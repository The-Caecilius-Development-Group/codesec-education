//! Contains core data structure types used in the database

use std::{fs::{self, File}, io};

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
    pub back: RichText
}

/// A set of flashcards for easy testing
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlashcardSet {
    /// Unique name of this set
    pub name: String,
    /// All the flashcards contained in this set
    pub flashcards: Vec<Flashcard>
}

/// All the user's save data
#[derive(Serialize, Deserialize)]
pub struct UserData {
    /// All of the user's flashcard sets
    pub sets: Vec<FlashcardSet>
}
/// A defualt user data i.e empty
impl Default for UserData {
    fn default() -> Self {
        Self { sets: vec![
            FlashcardSet {
                name: "French".into(),
                flashcards: vec![
                    Flashcard {
                        front: RichText::plaintext("I live".into()),
                        back: RichText::plaintext("J'habite".into())
                    },
                    Flashcard {
                        front: RichText::plaintext("I am".into()),
                        back: RichText::plaintext("Je suis".into())
                    },
                ]
            },
            FlashcardSet {
                name: "German".into(),
                flashcards: vec![
                    Flashcard {
                        front: RichText::plaintext("To eat".into()),
                        back: RichText::plaintext("essen".into())
                    },
                    Flashcard {
                        front: RichText::plaintext("Cockroach".into()),
                        back: RichText::plaintext("Kakerlaken".into())
                    },
                ]
            },
        ] }
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