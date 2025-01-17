//! Creates a RocksDB instance and ingests data from an SST file.
//!
//! # Overview
//! This module provides functionality to initialize a RocksDB database and ingest data from a 
//! pre-existing Sorted String Table (SST) file. It ensures the database directory exists before 
//! performing the ingestion and can create the database if it doesn't already exist.
use rocksdb::{DB, Options, IngestExternalFileOptions};
use std::fs;
use std::path::Path;


/// Creates a RocksDB instance and ingests data from an SST file.
///
/// # Arguments
/// - `db_path`: The path to the RocksDB database directory. If it does not exist, it will be created.
/// - `sst_path`: The path to the SST file to be ingested into the database.
///
/// # Returns
/// - `Ok(())`: If the database is successfully initialized and the SST file is ingested.
/// - `Err(Box<dyn std::error::Error>)`: If an error occurs during directory creation, database 
///   initialization, or SST file ingestion.
///
/// # Usage
/// ```rust
/// use your_crate::create_rocksdb_with_sst;
///
/// fn main() {
///     let db_path = "/path/to/rocksdb";
///     let sst_path = "/path/to/data.sst";
///
///     match create_rocksdb_with_sst(db_path, sst_path) {
///         Ok(_) => println!("RocksDB created and SST ingested successfully."),
///         Err(e) => eprintln!("Failed to create RocksDB or ingest SST: {}", e),
///     }
/// }
/// ```
pub fn create_rocksdb_with_sst(
    db_path: &str,
    sst_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the database path exists
    if !Path::new(db_path).exists() {
        println!("Creating database directory: {}", db_path);
        fs::create_dir_all(db_path)?;
    }

    println!("SST {}", sst_dir);

    let mut options = Options::default();
    options.create_if_missing(true);

    // Open the RocksDB database
    let db = DB::open(&options, db_path)?;
    let ingest_options = IngestExternalFileOptions::default();
    println!("Ingesting SST files into RocksDB at {}", db_path);
    // Ingest the SST files
    db.ingest_external_file_opts(&ingest_options, vec![sst_dir])?;

    println!("SST files have been ingested into RocksDB at {}", db_path);
    Ok(())
}
