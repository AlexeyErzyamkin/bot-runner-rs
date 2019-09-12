use std::path::{Path, PathBuf};
use std::io;
use std::fs;

use zip::{
    ZipWriter,
    ZipArchive,
    CompressionMethod,
    write::FileOptions
};

pub fn archive_data(path: &str, out_path: &str) -> io::Result<()> {
    let arc_file = fs::File::create(out_path)?;
    let mut zip = ZipWriter::new(arc_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored);

    arc_dir(Path::new(path), path, options, &mut zip)?;

    zip.finish()?;

    Ok(())
}

fn arc_dir<P: AsRef<Path>>(path: P, prefix: &str, options: FileOptions, mut zip: &mut ZipWriter<fs::File>) -> io::Result<()> {
    for each_file in fs::read_dir(path)? {
        let each_file = each_file?;
        let each_file_path = each_file.path();
        let file_type = each_file.file_type()?;

        let path = each_file_path.strip_prefix(Path::new(prefix)).expect("Can't strip prefix");

        if file_type.is_dir() {
            println!("   Dir: {}", path.display());

            zip.add_directory_from_path(path, options)?;

            arc_dir(each_file_path, prefix, options, &mut zip)?;
        } else if file_type.is_file() {
            println!("   File: {}", path.display());

            zip.start_file_from_path(path, options)?;
            let mut from_file = fs::File::open(each_file_path)?;
            io::copy(&mut from_file, &mut zip)?;
        }
    }

    Ok(())
}

pub fn unarchive_data(path: &str, out_path: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let mut archive = ZipArchive::new(file)?;

    for index in 0..archive.len() {
        let mut zip_file = archive.by_index(index)?;

        let mut path = PathBuf::from(out_path);
        path.push(zip_file.sanitized_name());

        if zip_file.name().chars().rev().next().map_or(false, |c| c == '/' || c == '\\') {
            println!("   Dir: {}", path.display());

            fs::create_dir_all(path)?;
        } else {
            if let Some(parent_dir) = path.parent() {
                fs::create_dir_all(parent_dir)?;
            }

            println!("   File: {}", path.display());

            let mut out_file = fs::File::create(path)?;
            io::copy(&mut zip_file, &mut out_file)?;
        }
    }

    Ok(())
}