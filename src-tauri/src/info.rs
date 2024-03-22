pub mod info {
    use crate::structures::structures::Login;
    use chrono::Local;
    use log::{error, info};
    use std::{
        io,
        path::PathBuf,
    };
    use toml;
    use walkdir::WalkDir;
    use whoami::{devicename, username};
    use tauri::Window;
    use crate::crypto::crypto_lib::{Encryption, Decrypt, Encoded};

    // Send user to frontend for help page
    #[tauri::command]
    pub async fn get_user_frontend(window: Window) {
        let user: String = get_user();
        window
            .emit("currentUser", user.clone())
            .expect("failed to emit user to frontend");
    }
    // Using whoami::username function to determine
    // current user for get_source_path()
    pub fn get_user() -> String {
        let user: String = username();
        user.trim().to_string()
    }

    pub fn get_pc_name() -> String {
        let pc_name: String = devicename();
        pc_name.trim().to_string()
    }
    // Gets size of path passed, shown here as parameter 'p'
    // Used to create Rust backend progress bars and frontend progress bar
    pub fn get_size(p: &PathBuf) -> u64 {
        let mut tb = 0; // tb = total bytes
        for ent in WalkDir::new(&p).follow_links(true) {
            match ent {
                // ent = entry
                Ok(fi) => {
                    if fi.file_type().is_file() {
                        // fi = file
                        tb += fi.metadata().map_or(0, |m| m.len());
                    }
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            }
        }

        tb // return total bytes
    }
    // Gets total size in bytes for updating frontend progress bar
    pub fn get_total(v: &Vec<Result<PathBuf, io::Error>>) -> f64 {
        let mut total = 0.0;
        for (_n, p) in v.iter().enumerate() {
            match p {
                Ok(path) => {
                    total += get_size(path) as f64;
                }
                Err(err) => {
                    eprintln!("error getting total {}", err);
                }
            }
        }
        
        total
    }
    pub fn get_datetime() -> String {
        let dt = Local::now();
        let dt_formatted = format!("{}", dt.format("%a, %b %d, %Y at %I:%M %p"));

        dt_formatted.clone()
    }

    pub async fn get_creds() -> Result<Login, reqwest::Error> {
        // let resource_path = handle
        //     .path_resolver()
        //     .resolve_resource("assets/assets")
        //     .expect("failed to resolve resource");

        let client = reqwest::Client::new();
        let request = client
            .get("http://10.2.1.57/creds")
            .header("accept", "application/json")
            .send()
            .await?;

        let response = request.text().await?;
        let mut lines = response.lines();

        let encoded = Encoded {
            key: String::from(lines.next().unwrap()),
            nonce: String::from(lines.next().unwrap()),
            message: String::from(lines.next().unwrap())
        };

        let toml = <Encryption as Decrypt>::decrypter(encoded).unwrap(); 
        let toml_str = toml.as_str();

        let creds: Login = match toml::de::from_str(&toml_str.trim()) {
            Ok(login) => {
                info!("Loaded credentials");
                login
            }
            Err(error) => {
                error!("Error loading credentials {:?}", error);
                panic!("Error loading credentials {:?}", error);
            }
        };

        println!("{:#?}", creds);
        Ok(creds) 
    }
}
