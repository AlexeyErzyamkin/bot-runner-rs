use std::fs;
use std::path::Path;
use std::fmt::Debug;
use std::io;

use zip::{
    ZipWriter,
    CompressionMethod,
    write::FileOptions
};

fn main() {
    let mut arc_file = fs::File::create("../arc.zip").unwrap();
    let mut zip = ZipWriter::new(arc_file);

    let path = "data";

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Deflated);

    arc_dir(path, path, options, &mut zip);

    zip.finish().unwrap();
}

fn arc_dir<P: AsRef<Path> + Debug>(path: P, prefix: &str, options: FileOptions, mut zip: &mut ZipWriter<fs::File>) {
    for each_file in fs::read_dir(path).unwrap() {
        let each_file = each_file.unwrap();
        let each_file_path = each_file.path();
        let file_type = each_file.file_type().unwrap();

        let path = each_file_path.strip_prefix(Path::new(prefix)).unwrap();

        if file_type.is_dir() {
            println!("Dir: {:?}", path);

            zip.add_directory_from_path(path, options).unwrap();

            arc_dir(each_file_path, prefix, options, &mut zip)
        } else if file_type.is_file() {
            println!("File: {:?}", path);
            println!("      {:?}", each_file_path);

            zip.start_file_from_path(path, options).unwrap();
            let mut from_file = fs::File::open(each_file_path).unwrap();
            io::copy(&mut from_file, &mut zip);
        }
    }
}

// fn arc_file<P: AsRef<Path> + Debug>(path: P, mut zip: &mut ZipWriter<fs::File>) {
//     dbg!(("file", path));
// }