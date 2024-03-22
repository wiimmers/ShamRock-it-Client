pub mod log_to_file {
    use std::path::PathBuf; 
    // Build log directory from current
    pub fn log_builder() -> PathBuf {
        format!("C:/Users/{}/AppData/Local/ShamRock-it-log/", whoami::username()).into()
    }
}