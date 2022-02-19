//! Contains core data structure types used in the database
use std::{time::Duration, time};
use std::{fs::{self, File}, io, ops::{Index, IndexMut}};

use log::{info, error};
use platform_dirs::AppDirs;
use serde::{Serialize, Deserialize};

/// Rich text - user inputted text with colour (for now).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RichText {
    pub text: String,
    pub color: String
}
impl RichText {
    /// An empty black text
    pub fn empty() -> Self {
        Self::plaintext("".into())
    }
    /// Plain, non-coloured text
    pub fn plaintext(text: String) -> Self {
        Self {text, color: "#000000".into()}
    }
}

/// A flashcard that will be shown to the user, with [`RichText`] on the front and back.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Flashcard {
    /// Text on the front of the flashcard
    pub front: RichText,
    /// Text on the back of the flashcard
    pub back: RichText,
    /// Flashcard id (used in dioxus keys)
    id: u64
}
impl Flashcard {
    /// Gets the id
    pub fn id(&self) -> u64 {
        self.id
    }
}

/// A set of flashcards for easy testing
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlashcardSet {
    /// Unique name of this set
    pub name: String,
    /// All the flashcards contained in this set
    pub flashcards: Vec<Flashcard>,
    /// Highest id (for adding cards)
    highest_id: u64
}
impl FlashcardSet {
    /// Creates a new, empty flashcard set
    pub fn new(name: String) -> Self {
        Self {name, flashcards: vec![], highest_id: 0}
    }
    /// Adds a flashcard to this set with the front and back [`RichText`]s
    pub fn add(&mut self, front: RichText, back: RichText) -> &Flashcard {
        let card = Flashcard {
            front, back, id: self.highest_id
        };
        self.flashcards.push(card);
        self.highest_id += 1;
        self.flashcards.last().unwrap()
    }
}
impl Index<u64> for FlashcardSet {
    type Output = Flashcard;

    fn index(&self, index: u64) -> &Self::Output {
        self.flashcards.iter().find(|f| f.id == index).expect("Invalid id")
    }
}
impl IndexMut<u64> for FlashcardSet {
    fn index_mut(&mut self, index: u64) -> &mut Self::Output {
        self.flashcards.iter_mut().find(|f| f.id == index).expect("Invalid id")
    }
}

/// All the user's save data
#[derive(Serialize, Deserialize)]
pub struct UserData {
    /// All of the user's flashcard sets
    pub sets: Vec<FlashcardSet>,
    
    pub duration_since_last_visit: Duration,

    pub last_visit: u64,

    pub last_sys_time: Duration,
}
/// A defualt user data i.e empty
impl Default for UserData {
    fn default() -> Self {
        let mut french = FlashcardSet::new("French".into());
        french.add(
        RichText::plaintext("I live".into()),
        RichText::plaintext("J'habite".into())
        );
        french.add(
                RichText::plaintext("I am".into()),
                RichText::plaintext("Je suis".into())
            );
        let mut german = FlashcardSet::new("German".into());
        german.add(
        RichText::plaintext("To eat".into()),
        RichText::plaintext("Essen".into())
        );
        german.add(
            RichText::plaintext("Cockroach".into()),
            RichText::plaintext("Kakerlaken".into())
        );
        Self {
            sets: vec![
                french, german
            ],
            duration_since_last_visit: Duration::ZERO,
            last_visit: 0,
            last_sys_time: Duration::ZERO

        }
    }
}
impl UserData {
    /// Save and serialise this data
    pub fn save(&self) -> io::Result<()> {
        info!("Saving user data");
        // Get some platform-specific save dirs
        let app_dirs = AppDirs::new(Some("magistrax"), true).unwrap();
        let data_path = app_dirs.data_dir.join("user-data.json");
        // Ensure all folders exist
        fs::create_dir_all(&app_dirs.data_dir)?;
        // Create (or truncate) & write
        let file = File::create(data_path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
    /// Load and deserialise this data
    pub fn load() -> io::Result<Self> {
        info!("Loading user data");
        // Get some platform-specific save dirs
        let app_dirs = AppDirs::new(Some("magistrax"), true).unwrap();
        let data_path = app_dirs.data_dir.join("user-data.json");
        let this: UserData;
        // Attempt to load this file
        if data_path.exists() {
            let file = File::open(data_path)?;
            this = serde_json::from_reader(file)?;
        } else {
            this = Self::default();
            this.save()?;
        }
        Ok (this)
    }
}

/// Encapsulates [`UserData`] to ensure it is saved after modification.
pub struct UserDataAccessor {
    data: UserData
}
impl UserDataAccessor {
    pub fn new(data: UserData) -> Self {
        Self {data}
    }
    pub fn get(&self) -> &UserData {
        &self.data
    }
    pub fn modify(&mut self, f: impl FnOnce(&mut UserData)) {
        f(&mut self.data);
        if let Err(e) = self.data.save() {
            error!("Could not save data: {}", e);
        }
    }
}