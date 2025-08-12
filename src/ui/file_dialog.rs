use std::path::PathBuf;

pub struct FileDialog;

impl FileDialog {
    pub fn new() -> Self {
        Self
    }

    pub fn open_file(&self) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("JSON files", &["json"])
            .set_title("Select DSA Character File")
            .pick_file()
    }
}