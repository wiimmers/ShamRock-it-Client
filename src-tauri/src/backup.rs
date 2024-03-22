#[macro_use]
pub mod backup {
    use std::{
        fs::{self, File},
        io::{self, Read, Write},
        path::{Path, PathBuf},
    };
    use tauri::Window;
    use walkdir::WalkDir;
    use zip::{write::FileOptions, write::ZipWriter};

    use crate::info::info::{get_size, get_user, get_total};

    /* CONSTANTS */
    // Pre formated start path for file system
    // Adjust if user files are on a different drive
    const START_PATH: &str = "C:\\";
    // Namespace for deflated compression method
    const DEFLATED: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);
    // Folders array for creating destination folders
    static FOLDERS: [&str; 6] = [
        "Desktop",
        "Documents",
        "Downloads",
        "Favorites",
        "Pictures",
        "Videos",
    ];
    // Emits values to frontend for progress bar
    #[tauri::command]
    pub fn update_progress_bar(position: u64, total: f64, window: Window) {
        window
            .emit("progressTotal", total)
            .expect("failure to pass total_mb");
        window
            .emit("updateProgress", position as f64)
            .expect("Failed to emit progress position");
    }
    // Backup function
    #[tauri::command]
    pub async fn backup_user(window: Window) {
        // Make the destination directory at destination path
        let _ = make_dir(&get_destination_path());

        let mut source_paths: Vec<Result<PathBuf, io::Error>> = Vec::new();
        let mut destination_paths: Vec<PathBuf> = Vec::new();
        // Initialize source paths and destination paths
        init_source_paths(&mut source_paths);
        init_destination_paths(&mut destination_paths);
        // Append source folders to current user path
        // and create duplicate paths in destination vector
        append_source_paths(&mut source_paths);
        append_destination_paths(&mut destination_paths);

        let size = get_total(&source_paths);

        for (n, p) in destination_paths.iter().enumerate() {
            let _ = copy_files(&source_paths[n].as_mut().unwrap(), p, size, window.clone());
        }
        // Zip destination folders
        let _ = zip_dir(&mut get_destination_path(), window.clone());
        // Delete copied folder
        let _ = delete_destination(&mut get_destination_path(), window.clone());
    }
    // Format destination path for creation of directories
    fn get_destination_path() -> PathBuf {
        let tmp = "tmp";
        let end_user = get_user();
    
        Path::new(START_PATH)
            .join(&tmp)
            .join(&end_user)
            .to_path_buf()
    } 
    // Make directory from passed parameter "init_dir",
    // init_dir = result of get_destination_path()
    fn make_dir(init_dir: &PathBuf) -> std::io::Result<()> {
        if let Err(err) = fs::create_dir_all(&init_dir) {
            eprintln!("Error creating directory: {:?}", err);
            return Err(err);
        }

        // Set permissions to prevent inability to create zip file
        // and prevent disallowing file transfer
        let mut perms = fs::metadata(init_dir)?.permissions();
        perms.set_readonly(false);
        fs::set_permissions(&init_dir, perms)?;

        // Sends newly created init_dir to have destination
        // folders created as children directories

        Ok(())
    }
    // Get source path to fill vector 'source_paths' in main
    fn get_source_path() -> Result<PathBuf, io::Error> {
        let user = get_user();
        const USERS: &str = "Users";

        let source_path = Path::new(START_PATH).join(&USERS).join(&user).to_path_buf();

        if source_path.exists() {
            Ok(source_path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Source path does not exist.",
            ))
        }
    }
    fn init_source_paths(source_paths: &mut Vec<Result<PathBuf, io::Error>>) {
        // Create default current user path i.e. "C:\\Users\\Bob" in new vector
        // Vector will have same size as const FOLDERS
        for _ in 0..FOLDERS.len() {
            source_paths.push(get_source_path());
        }
    } 
    fn init_destination_paths(destination_paths: &mut Vec<PathBuf>) {
        // Create destination paths mimicking source
        // Vector will have same size as const FOLDERS
        for _ in 0..FOLDERS.len() {
            destination_paths.push(get_destination_path());
        }
    }
    // Takes default paths in vector 'source_paths' and passes it here
    // as 'source_paths_vec', appends 'FOLDERS' to the end of each entry
    fn append_source_paths(src_paths_vec: &mut Vec<Result<PathBuf, io::Error>>) {
        for (n, result) in src_paths_vec.iter_mut().enumerate() {
            match result {
                Ok(path) => {
                    // Clone path and modify clone
                    let mut modified_path = path.clone();
                    if let Some(folder) = FOLDERS.get(n) {
                        modified_path.push(Path::new(folder));
                    }
                    // Replace original path with the modified path
                    *result = Ok(modified_path)
                }
                Err(err) => {
                    eprintln!("{:?}", err)
                }
            }
        }
    }
    fn append_destination_paths(dst_paths_vec: &mut Vec<PathBuf>) {
        for (n, pb) in dst_paths_vec.iter_mut().enumerate() {
            let mut modified_path = pb.clone();
            if let Some(folder) = FOLDERS.get(n) {
                modified_path.push(Path::new(folder));
            }
            // Replace original path with the modified path
            *pb = modified_path;
        }
    }
    // Copies files from source path to destination path
    // Takes total size and window object for later passing to update_progress_bar
    fn copy_files(
        src_pth: &PathBuf,
        dst_pth: &PathBuf,
        size: f64,
        window: Window,
    ) -> Result<(), std::io::Error> {
        println!(
            "Copying files from {} to {}",
            src_pth.display(),
            dst_pth.display()
        );
        window.emit("beginCopy", 0).expect("did not begin copy");

        for from in WalkDir::new(src_pth) {
            let from = match from {
                Ok(copy_ent) => copy_ent,
                Err(err) => {
                    eprintln!("{:?}", err);
                    continue;
                }
            };

            let copy_pth = from.path();

            // Skip desktop.ini files
            if copy_pth
                .file_name()
                .map(|s| s.to_string_lossy() == "desktop.ini")
                .unwrap_or(false)
            {
                continue;
            }

            let copy_fi_name = copy_pth.strip_prefix(src_pth).unwrap();
            let to = dst_pth.join(copy_fi_name);

            if from.file_type().is_file() {
                match fs::create_dir_all(to.parent().unwrap()) {
                    Ok(_) => {
                        if let Err(err) = fs::copy(copy_pth, &to) {
                            eprintln!("Error copying file {:?}: {:?}", copy_pth, err);
                        } else {
                            update_progress_bar(from.metadata()?.len(), size, window.clone());
                        }
                    }
                    Err(err) => eprintln!(
                        "error creating directories for {:?}: {:?}",
                        &to.parent().unwrap(),
                        err
                    ),
                }
            } else if from.file_type().is_dir() {
                let _ = fs::create_dir_all(&to);
            }
        }

        window.emit("beginCopy", 1).expect("did not complete copy");

        Ok(())
    }
    // Creates zip file
    // Reads files that are children of 'destination_path'
    // Compresses and moves them to newly created zip file
    fn zip_dir(destination_path: &PathBuf, window: Window) -> std::io::Result<()> {
        window.emit("beginZip", 0).expect("did not begin zip");
        // Takes destination path and appends '.zip' to the end
        let mut zip_path = destination_path.clone();
        zip_path.set_extension("zip");

        // Creates zip file at new 'zip_path'
        let zip_file = File::create(&zip_path).expect("Failed to create zip file");
        let mut perms = fs::metadata(&zip_path)?.permissions();
        perms.set_readonly(false);
        fs::set_permissions(&zip_path, perms)?;

        // Initialize 'ZipWriter' method and create new ZipWriter struct
        let mut zip_writer = ZipWriter::new(&zip_file);
        let zip_options = FileOptions::default().compression_method(DEFLATED.unwrap()); //DEFLATED namespace declared above

        println!("Zipping");
        println!("Output file at path: {:?}", &zip_path);

        // Recursively find directories and their files
        for entry in WalkDir::new(destination_path) {
            let (path, name) = match entry {
                // If result is Ok, set path and name accordingly (name is path stripped of prefix)
                Ok(dir_entry) => {
                    let path = dir_entry.path().to_owned();
                    let name = path
                        .strip_prefix(Path::new(destination_path))
                        .unwrap()
                        .to_owned();
                    (path, name)
                }
                // Error reporting
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    continue;
                }
            };
            // If the path == file is true
            if path.is_file() {
                // Add new 'zipped' file to compress data to zip file, giving same name
                #[allow(deprecated)]
                zip_writer
                    .start_file_from_path(&name /* Same name as original */, zip_options)
                    .expect("Error adding file from path: {name:?}");
                // Open file using defined path
                let mut f = File::open(path).expect("Cannot open file for zip writer: {name:?}");
                let mut buffer = Vec::new();
                // Read all bytes in opened file and place in buffer
                f.read_to_end(&mut buffer)
                    .expect("Cannot read file to end: {name:?}");
                update_progress_bar(
                    f.metadata()?.len(),
                    get_size(&destination_path) as f64,
                    window.clone(),
                );
                // Write contents of buffer, populated above, to compressed file
                zip_writer
                    .write_all(&buffer)
                    .expect("Cannot write file: {name:?}");
                // Clear buffer
                buffer.clear();
            } else if !name.as_os_str().is_empty() {
                // If the entry is not a file add it as a directory, i.e. "Desktop"
                // Write directory in zip file
                #[allow(deprecated)]
                zip_writer
                    .add_directory_from_path(&name /* Same name as original */, zip_options)
                    .expect("Error adding directory from path");
            }
        }
        // Finish zipping and handle error case
        zip_writer.finish().expect("Failed to finalize ZIP file");
        // Alert user that compression is complete
        println!("Compression of {destination_path:?} complete!");

        window.emit("beginZip", 1).expect("did not complete zip");
        Ok(())
    }
    // Deletes large directory created as buffer for zipping
    // Located at destination path, named del here
    fn delete_destination(del: &PathBuf, window: Window) -> std::io::Result<()> {
        window.emit("beginDelete", 0).expect("did not begin delete");
        println!("Deleting {del:?}");
        fs::remove_dir_all(del)?;

        window
            .emit("beginDelete", 1)
            .expect("did not complete delete");
        Ok(())
    }
}
