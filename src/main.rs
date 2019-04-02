extern crate failure;
use std::fs::{read_dir, metadata};
use std::result::Result;
use failure::Error;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Eq, PartialEq, Hash)]
struct FileMetadata {
    path: String,
    size: u64
}

impl FileMetadata {
    fn new(path : String, size : u64) -> FileMetadata {
        FileMetadata { path, size }
    }
}


#[derive(Debug)]
struct HashRecord {
    set : HashSet<FileMetadata>
}

impl HashRecord {
    fn new() -> HashRecord {
        HashRecord { set : HashSet::new() }
    }

    fn hash_dir(&mut self, dir: &Path) -> Result<(), Error> {
        self._hash_dir(dir, dir.to_str().unwrap())
    }

    fn _hash_dir(&mut self, dir: &Path, root : &str) -> Result<(), Error> {
        if dir.is_dir() {
            for entry in read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let size = metadata(&path)?.len();
                    let path = path.to_str().unwrap().to_string().replace(root, "");
                    self.set.insert(FileMetadata::new(path, size));
                }
                if path.is_dir() {
                    return self._hash_dir(&path, root);
                }
            }
        }
        Ok(())
    }
}


fn main() -> Result<(), Error> {
    let mut record = HashRecord::new();
    record.hash_dir(Path::new("C:\\Users\\alex\\Documents\\Books"))?;
    let mut record2 = HashRecord::new();
    record2.hash_dir(Path::new("D:\\56862"))?;
    println!("{:?}", record.set.difference(&record2.set));
    Ok(())
}
