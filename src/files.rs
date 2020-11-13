use std::path::Path;
use std::fs;
use std::io::Read;
use std::slice::Iter;

pub struct FileData {
    file_name: String,
    file_path: String,
}

impl FileData {
    fn new(name: String, path: String) -> FileData {
        return FileData { file_name: name, file_path: path };
    }

    pub fn name(&self) -> &String {
        return &self.file_name;
    }

    pub fn path(&self) -> &String {
        return &self.file_path;
    }
}

pub struct DirFiles {
    files: Vec<FileData>,
}

impl DirFiles {
    fn new() -> DirFiles {
        return DirFiles { files: Vec::new() };
    }

    fn push(&mut self, file: FileData) {
        self.files.push(file);
    }

    fn list() {}

    pub fn iter(&self) -> Iter<'_, FileData> {
        return self.files.iter();
    }
}

pub fn file(base_folder: String) -> DirFiles {
    let mut files = DirFiles::new();

    // Create a path to Serve Directory
    let path: &Path = Path::new(&base_folder);

    if path.exists() && path.is_dir() {
        let paths = fs::read_dir(path).unwrap();
        for route in paths {
            let entry = route.unwrap();
            files.push(FileData::new(entry.file_name().as_os_str().to_str().unwrap().to_string(),
                                     entry.path().to_str().unwrap().to_string()));
        }
    }
    return files;
}

