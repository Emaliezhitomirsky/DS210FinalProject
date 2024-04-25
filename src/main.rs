use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;

struct NameData {
    year: u32,
    gender: String,
    ethnicity: String, 
    name: String, 
    count: u32,
    rank: u32,
}

impl NameData {
    fn from_record(record: csv::StringRecord) -> Option<Self> {
        if record.len() == 6 {
            Some(NameData {
                year: record[0].parse().ok()?,
                gender: record[1].to_string(),
                ethnicity: record[2].to_string(),
                name: record[3].to_string(), 
                count: record[4].parse().ok()?,
                rank: record[5].parse().ok()?,
            })
        } else {
            None
        }
    }
}
fn main() {
    
}

// Test test test 