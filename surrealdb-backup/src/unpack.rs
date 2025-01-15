use rocksdb::{DB, Options, IngestExternalFileOptions};
use std::fs;
use std::path::Path;


pub fn create_rocksdb_with_sst(
    db_path: &str,
    sst_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the database directory exists
    if !Path::new(db_path).exists() {
        fs::create_dir_all(db_path)?;
    }

    // Open or create the RocksDB instance
    let mut options = Options::default();
    options.create_if_missing(true);

    let db = DB::open(&options, db_path)?;

    // Ingest the SST file into the RocksDB instance
    let ingest_options = IngestExternalFileOptions::default();
    db.ingest_external_file_opts(&ingest_options, vec![sst_path])?;

    println!("SST file has been ingested into RocksDB at {}", db_path);
    Ok(())
}
