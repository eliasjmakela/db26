use std::io;
use std::io::prelude::*;
use std::fs::File;


fn main() -> io::Result<()> {
    put("vau", "vou")?;
    put("hei", "moi")?;
    Ok(())
}


fn put(key: &str, val: &str) -> io::Result<()> {
    let mut db = File::options()
        .append(true)
        .create(true)
        .open("database.db26")?;
    db.write_all(key.as_bytes())?;
    db.write_all(":".as_bytes())?;
    db.write_all(val.as_bytes())?;
    db.write_all(";\n".as_bytes())?;
    Ok(())
}
