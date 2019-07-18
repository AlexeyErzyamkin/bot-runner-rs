use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufReader;

use serde::de::DeserializeOwned;

pub fn read_config<'a, T, P>(path: P) -> io::Result<T>
    where T: DeserializeOwned,
          P: AsRef<Path>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(obj) => Ok(obj),
        Err(_) => Err(io::ErrorKind::InvalidData.into())
    }
}