// src/main.rs
use chrono::Local;
use clap::{Parser, Subcommand};
use serde_json::{from_reader, to_writer};
use std::fs::{File, OpenOptions};
use std::io::BufReader;

mod models;
use models::MoodEntry;

#[derive(Parser)]
#[command(name = "Mood Tracker")]
#[command(about = "Track your daily moods", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new mood entry
    Add {
        /// Mood level (e.g., 1-5)
        mood: String,

        /// Additional notes
        #[arg(short, long)]
        notes: Option<String>,

        /// Tags for categorizing the mood entry
        #[arg(short, long)]
        tags: Vec<String>,
    },
    /// View all mood entries
    View,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { mood, notes, tags } => {
            add_mood_entry(mood.clone(), notes.clone(), tags.clone());
        }
        Commands::View => {
            view_entries();
        }
    }
}

fn add_mood_entry(mood: String, notes: Option<String>, tags: Vec<String>) {
    let entry = MoodEntry {
        date: Local::now().format("%Y-%m-%d").to_string(),
        mood,
        notes,
        tags,
    };

    let file = OpenOptions::new().append(true).create(true).open("mood_entries.json").unwrap();
    to_writer(&file, &entry).expect("Failed to write entry");
    println!("Mood entry added: {:?}", entry);
}

fn view_entries() {
    let file = File::open("mood_entries.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let entries: Vec<MoodEntry> = from_reader(reader).expect("Could not read entries");

    for entry in entries {
        println!("{} - Mood: {} [{}]", entry.date, entry.mood, entry.tags.join(", "));
        if let Some(notes) = &entry.notes {
            println!("  Notes: {}", notes);
        }
    }
}
