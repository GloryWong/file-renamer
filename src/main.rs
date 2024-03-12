use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let matches = Command::new("File Renamer")
        .version("0.1.0")
        .author("Glory Wong")
        .about("Renames files in a directory based on a pattern")
        .arg(Arg::new("path")
            .short('p')
            .long("path")
            .value_name("DIR_PATH")
            .help("Sets the path to the directory")
            .required(true))
        .get_matches();

    let path = matches.get_one::<String>("path").expect("Path argument missing");
    println!("Renaming files in directory: {}", path);

    // Proceed with the renaming logic, passing `path` as a `&Path`
    if let Err(e) = rename_files(Path::new(path)) {
        eprintln!("Error: {}", e);
    }
}

fn rename_files(dir_path: &Path) -> Result<(), std::io::Error> {
    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = match entry {
                Ok(file) => file,
                Err(e) => return Err(e)
            };
            let path = entry.path();
            if path.is_file() {
                // Generate a timestamp string
                let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
                let timestamp_str = format!("{}", timestamp.as_secs());

                // Construct the new file name with the timestamp
                if let Some(file_stem) = path.file_stem() {
                    if let Some(extension) = path.extension() {
                        let new_file_name = format!("{}_{}.{}", file_stem.to_string_lossy(), timestamp_str, extension.to_string_lossy());
                        let new_path = path.with_file_name(new_file_name);
                        fs::rename(&path, &new_path)?;
                        println!("File renamed from {:?} to {:?}", path, new_path);
                    } else {
                        // If there's no extension, just append the timestamp to the file stem
                        let new_file_name = format!("{}_{}", file_stem.to_string_lossy(), timestamp_str);
                        let new_path = path.with_file_name(new_file_name);
                        fs::rename(&path, &new_path)?;
                        println!("File renamed from {:?} to {:?}", path, new_path);
                    }
                }
            }
        }
    } else {
        println!("The path specified is not a directory");
    }

    Ok(())
}
