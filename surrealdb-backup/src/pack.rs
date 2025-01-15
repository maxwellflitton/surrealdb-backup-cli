use rocksdb::{DB, Options, SstFileWriter};


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
