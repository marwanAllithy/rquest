use std::{
    fs,
    path::{Path, PathBuf},
};

use dirs::data_local_dir;
use serde_json::from_str;

use crate::sidebar::Collection;

fn get_data_path() -> PathBuf {
    let mut path = data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("rquest");
    path.push("data.json");
    path
}

// Initialize the data directory and file
pub fn init_data_file() -> std::io::Result<()> {
    let path = get_data_path();

    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create empty file if it doesn't exist
    if !path.exists() {
        let empty_collections: Vec<Collection> = Vec::new();
        let json = serde_json::to_string_pretty(&empty_collections)?;
        fs::write(&path, json)?;
    }

    Ok(())
}

pub fn fetch_collections() -> std::io::Result<Vec<Collection>> {
    let path = ".local/share/reqwest/data.json";
    let data = fs::read_to_string(path)?;
    let res: Vec<Collection> = from_str(&data)?;
    Ok(res)
}

pub fn fetch_collection() {}

pub fn add_collection(new_collection: Collection) -> std::io::Result<()> {
    let path = get_data_path();
    init_data_file()?;

    let data = fs::read_to_string(&path)?;
    let mut collections: Vec<Collection> = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    collections.push(new_collection);

    let new_data = serde_json::to_string_pretty(&collections)?;
    fs::write(path, new_data)?;

    Ok(())
}

pub fn edit_collectiom() {}
