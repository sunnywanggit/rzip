extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use std::fs;
use std::io::{copy, Read, Seek, Write};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::time::Instant;

use walkdir::{ DirEntry,WalkDir };
use zip::write::FileOptions;
use zip::result::ZipError;


# [wasm_bindgen]
pub fn zip() {
    let now = Instant::now();
    let zip_result = compress_dir(Path::new("./files"),Path::new("./result.zip"));
    if let Ok(()) = zip_result{
        println!("run time: {}", now.elapsed().as_millis());
    }else {
        println!("error??{:?}",zip_result)
    }
}

pub fn compress_dir(src_dir: &Path, target: &Path) ->std::result::Result<(),Box<dyn std::error::Error>>{
    let zipfile = std::fs::File::create(target)?;
    let dir = WalkDir::new(src_dir);
    zip_dir(&mut dir.into_iter().filter_map(|e|e.ok()),
            src_dir.to_str().unwrap(),
            zipfile)?;
    Ok(())
}

#[warn(dead_code)]
pub fn compress_file(src_dir: &Path, target: &Path)->std::result::Result<(),Box<dyn std::error::Error>>{
    let zipfile = std::fs::File::create(target)?;
    let dir = WalkDir::new(src_dir);

    let prefix = src_dir.parent().map_or_else(||"/",|p|p.to_str().unwrap());
    zip_dir(&mut dir.into_iter().filter_map(|e|e.ok()),prefix,zipfile)?;
    Ok(())
}


pub fn zip_dir<T>(it: &mut dyn Iterator<Item=DirEntry>,
              prefix: &str,
              writer:T) ->zip::result::ZipResult<()>
    where T: Write + Seek{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);
    let mut buffer = Vec::new();
    for entry in it{
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        if path.is_file(){
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file(name.to_string_lossy(),options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        }else if name.as_os_str().len() != 0 {
            zip.add_directory(name.to_string_lossy(),options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}
