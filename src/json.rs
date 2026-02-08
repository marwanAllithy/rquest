use std::{
    fs,
    path::{Path, PathBuf},
};

use dirs::data_local_dir;
use serde_json::from_str;

use crate::sidebar::{Collection, RequestStructs};

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
    let path = get_data_path();
    init_data_file()?;

    let data = fs::read_to_string(&path)?;
    let collections: Vec<Collection> = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(collections)
}

pub fn fetch_collection_by_index(index: usize) -> std::io::Result<Collection> {
    let path = get_data_path();
    init_data_file()?;
    let data = fs::read_to_string(&path)?;
    let collections: Vec<Collection> = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    collections.get(index).cloned().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Collection at index {} not found", index),
        )
    })
}

pub fn fetch_collection(uuid: String) -> std::io::Result<Collection> {
    let path = get_data_path();
    init_data_file()?;

    let data = fs::read_to_string(&path)?;
    let collections: Vec<Collection> = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    collections
        .into_iter()
        .find(|c| c.id == uuid)
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find collection")
        })
    //Ok()
}

pub fn add_request(collection_id: String, new_request: RequestStructs) -> std::io::Result<()> {
    let path = get_data_path();
    init_data_file()?;

    let data = fs::read_to_string(&path)?;
    let mut collections: Vec<Collection> = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let collection = collections
        .iter_mut()
        .find(|c| c.id == collection_id)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Collection not found"))?;

    collection.requests.push(new_request);

    let json = serde_json::to_string_pretty(&collections)?;
    fs::write(path, json)?;

    Ok(())
}

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

pub fn del_request(collection_id: String, index: usize) -> std::io::Result<()> {
    let path = get_data_path();
    init_data_file()?;

    let data = fs::read_to_string(&path)?;
    let mut collections: Vec<Collection> = serde_json::from_str(&data)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let collection = collections
        .iter_mut()
        .find(|c| c.id == collection_id)
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Collection not found"))?;

    // Check if index is valid
    if index >= collection.requests.len() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Request index out of bounds",
        ));
    }

    // Remove the request at the index
    collection.requests.remove(index);

    let json = serde_json::to_string_pretty(&collections)?;
    fs::write(path, json)?;

    Ok(())
}

pub fn edit_collection() {}
