// src/models.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MoodEntry {
    pub date: String,
    pub mood: String,
    pub notes: Option<String>,
    pub tags: Vec<String>,
}
