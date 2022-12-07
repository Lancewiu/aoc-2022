#[derive(Debug)]
pub struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Default, Debug)]
pub struct Directory {
    directories: Vec<String>,
    files: Vec<File>,
}

impl Directory {
    pub fn add_directory(&mut self, name: &str) {
        self.directories.push(String::from(name));
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn get_file_sizes(&self) -> usize {
        self.files.iter().map(|file| file.get_size()).sum()
    }

    pub fn get_subdirectory_names(&self) -> &[String] {
        &self.directories[..]
    }
}
