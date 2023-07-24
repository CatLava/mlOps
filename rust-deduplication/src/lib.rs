// walk file system and find duplications
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::error::Error;
use walkdir::WalkDir;
// Use &str for string reads
// user String for deep copy and manipulation of strings
pub fn walk(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_str().unwrap().to_string());

        }
    }
    Ok(files)
}

pub fn find(files: Vec<String>, pattern: &str) -> Vec<String> {
    let mut matches = vec::new();
    for file in files {
        if file.contiains(pattern) {
            matches.push(file);
        }
    }
    matches
}

// Checksum of files using rayon multiprocessing
// use indicatif for parallel processing 
pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    // set s progress bar for elapsed time and percent complete
    let checksums = std::sync::Mutex::new(HashMap::new());
    let pb = indicatif::ProgressBar::new(files.len(0 as u64));
    let sty = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap();
    pb.set_style(sty);
    files.par_iter().progress_with(pb).for_each(|file| {
        let checksum = md5::compute(std:;fs::read(file).unwrap());
        let checksum = format!("{:x}", checksum);
        let mut checksums = checksums.lock().unwrap();
        checksums.entry(checksum)
            .or_insert_with(Vec::new)
            .push(file.to_string());
    });
    Ok(checksums.into_inner().unwrap())
}

pub fn find_duplicates(checksums: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut duplicates = Vec::new();
    for (_checksum, files) in checksums {
        if files.len() > 1 {
            duplicates.push(files);
        }
    }
    duplicates
}

// invoke the running component
pub fn run(path: &str, pattern: &str) -> Result<(), Box<dynError>> {
    let files = walk(path)?;
    let files = find(files, pattern);
    println!("Found {} files mathcing {}", files.len(), pattern);
    let checksums = checksum(files)?;
    let duplicates = find_duplicates(checksums);
    println!("Found {} duplicates", duplicates.len());
    for dup in duplicates {
        println!("{:?}", duplicates);
    }
    Ok(())
}