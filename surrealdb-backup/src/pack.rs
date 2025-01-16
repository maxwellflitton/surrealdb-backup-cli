//! Exports data from a RocksDB database to an SST file.
//!
//! # Overview
//! This module defines a function to export all key-value pairs from a RocksDB database into a 
//! Sorted String Table (SST) file. This operation is useful for creating portable backups or 
//! transferring data between environments.
//!
//! # Features
//! - Reads key-value pairs from a RocksDB database.
//! - Writes the data to a new SST file using RocksDB's `SstFileWriter`.
//! - Handles errors gracefully and returns detailed error information.
//!
//! # Dependencies
//! - The `rocksdb` crate is used for interacting with RocksDB databases and creating SST files.
use rocksdb::{DB, Options, SstFileWriter};

/// Exports data from a RocksDB database to an SST file.
///
/// # Arguments
/// - `db_path`: The path to the existing RocksDB database directory.
/// - `sst_path`: The path where the SST file will be created and stored.
///
/// # Returns
/// - `Ok(())`: If the export operation is successful.
/// - `Err(Box<dyn std::error::Error>)`: If an error occurs during the process.
///
/// # Usage
/// ```rust
/// use your_crate::export_to_sst;
///
/// fn main() {
///     let db_path = "/path/to/rocksdb";
///     let sst_path = "/path/to/export.sst";
///
///     match export_to_sst(db_path, sst_path) {
///         Ok(_) => println!("Data exported successfully to {}", sst_path),
///         Err(e) => eprintln!("Failed to export data: {}", e),
///     }
/// }
/// ```
pub fn export_to_sst(db_path: &str, sst_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::open_default(db_path)?;
    let options = Options::default();
    let mut writer = SstFileWriter::create(&options);
    writer.open(sst_path)?;

    for x in db.iterator(rocksdb::IteratorMode::Start) {
        let (key, value) = x?;
        writer.put(key, value)?;
    }
    writer.finish()?;
    Ok(())
}
