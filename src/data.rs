//! Contains core data structure types used in the database

use std::{fs::{self, File}, io};
use std::time::Duration;

use log::info;
use platform_dirs::AppDirs;
use serde::{Serialize, Deserialize};

/// Represents formatting that can be applied to any rich text
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum RichTextFormat {
    Bold, Italic, Underlined
}

/// A node of rich text, containing some formatting
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RichTextNode (String, Vec<RichTextFormat>);

/// Rich text - user inputted text with formatting. Contains several nodes each with their own styles
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RichText (Vec<RichTextNode>);
impl RichText {
    /// Plain, non-formatted text
    pub fn plaintext(text: String) -> Self {
        Self (vec![RichTextNode(text, vec![])])
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
    pub fn new(name: String) -> Self {
        Self {name, flashcards: vec![], highest_id: 0}
    }
    pub fn add(&mut self, front: RichText, back: RichText) -> &mut Self {
        self.flashcards.push(Flashcard {
            front, back, id: self.highest_id
        });
        self.highest_id += 1;
        self
    }
}

/// All the user's save data
#[derive(Serialize, Deserialize)]
pub struct UserData {
    /// All of the user's flashcard sets
    pub sets: Vec<FlashcardSet>,
    
    pub duration_since_last_visit: Duration,

    pub last_visit: u32,

    pub current_time: u32,
}
/// A defualt user data i.e empty
impl Default for UserData {
    fn default() -> Self {
        let mut french = FlashcardSet::new("French".into());
        french
            .add(
            RichText::plaintext("I live".into()),
            RichText::plaintext("J'habite".into())
            )
            .add(
                RichText::plaintext("I am".into()),
                RichText::plaintext("Je suis".into())
            );
        let mut german = FlashcardSet::new("German".into());
        german
            .add(
            RichText::plaintext("To eat".into()),
            RichText::plaintext("Essen".into())
            )
            .add(
                RichText::plaintext("Cockroach".into()),
                RichText::plaintext("Kakerlaken".into())
            );
        Self {
            sets: vec![
                french, german
            ],
            duration_since_last_visit: Duration::ZERO,
            last_visit: 0,
            current_time: 0

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