extern crate bincode;
extern crate rustc_serialize;
use num_enum::TryFromPrimitive;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use std::{thread, time};

const MAX_OPEN_FILES: usize = 5;

#[derive(RustcEncodable, RustcDecodable, Clone, Eq, PartialEq)]
pub enum SystemResult {
    Ok = 0,
    Exit = -1,
    FileAlreadyExists = -4,
    FileDoesntExist = -5,
    FileNotOpen = -8,
    FileAlreadyOpen = -9,
    OpenInInvalidMode = -10,
    PermissionDenied = -6,
    ReachedMaxOpenFiles = -7,
    MiscellaneousError = -11,
    ClientAlreadyExists = -12,
    ClientDoesntExist = -13,
    WrongCredentials = -14,
}

#[derive(RustcEncodable, RustcDecodable, Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
pub enum Permission {
    None = 0,      // 0b0000
    Write = 1,     // 0b0001
    Read = 2,      // 0b0010
    ReadWrite = 3, // 0b0011
}

#[derive(RustcEncodable, RustcDecodable, Clone, Debug)]
pub struct _File {
    name: String,
    content: String,
    owner_permission: Permission,
    others_permission: Permission,
}
#[derive(RustcEncodable, RustcDecodable, Clone, Debug)]
pub struct OpenFile {
    file_name: String,
    permission: Permission,
}
#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct FileSystem {
    files: HashMap<String, _File>,
    open_files: Vec<OpenFile>,
}

impl _File {
    pub fn new(
        name: String,
        content: String,
        owner_permission: Permission,
        others_permission: Permission,
    ) -> _File {
        _File {
            name,
            content,
            owner_permission,
            others_permission,
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            files: HashMap::default(),
            open_files: Vec::new(),
        }
    }

    pub fn get_file(&self, file_name: &str) -> &_File {
        self.files.get(file_name).unwrap()
    }

    pub fn add_file(&mut self, file: _File) -> SystemResult {
        match self.filename_exists(&file.name) {
            Some(_) => {
                thread::sleep(time::Duration::new(1, 100_000_000));
            }
            None => {
                self.files.insert(String::from(&file.name), file);
            }
        }

        SystemResult::Ok
    }

    pub fn delete_file(&mut self, file_name: String) -> SystemResult {
        match self.filename_exists(&file_name) {
            Some(file) => match self.file_is_open(&file) {
                Some((fd, _)) => {
                    self.close_file(fd);
                }
                None => {}
            },
            None => {
                return SystemResult::FileDoesntExist;
            }
        };

        self.files.remove(&file_name);

        SystemResult::Ok
    }

    pub fn rename_file(&mut self, file_name: String, new_file_name: String) -> SystemResult {
        let old_file: &_File;

        match self.filename_exists(&file_name) {
            // Check if the file to rename exists
            Some(file) => old_file = file,
            None => return SystemResult::FileDoesntExist,
        }
        match self.filename_exists(&new_file_name) {
            // Check if the new file name exists
            Some(_) => return SystemResult::FileAlreadyExists,
            None => {}
        }

        let new_file = _File {
            // Create new file from old file with replaced content
            name: new_file_name.clone(),
            content: old_file.content.clone(),
            owner_permission: old_file.owner_permission,
            others_permission: old_file.others_permission,
        };

        let opt: Option<Permission> = match self.file_is_open(old_file) {
            Some((_, open_file)) => Some(open_file.permission),
            None => None,
        };

        self.delete_file(file_name);
        self.add_file(new_file.clone());

        if let Some(open_permission) = opt {
            self.open_file(new_file_name, open_permission);
        };

        SystemResult::Ok
    }

    pub fn open_file(&mut self, file_name: String, open_permission: Permission) -> SystemResult {
        if self.open_files.len() == MAX_OPEN_FILES {
            return SystemResult::ReachedMaxOpenFiles;
        }
        let file_name_to_open;
        match self.filename_exists(&file_name) {
            Some(file) => {
                match self.file_is_open(&file) {
                    Some((_, _)) => return SystemResult::FileAlreadyOpen,
                    None => {
                        if !self.check_permission(&open_permission, &file) {
                            return SystemResult::PermissionDenied;
                        }
                    }
                };
                file_name_to_open = file.name.clone();
            }
            None => return SystemResult::FileDoesntExist,
        };

        self.open_files.push(OpenFile {
            file_name: file_name_to_open,
            permission: open_permission,
        });

        SystemResult::Ok
    }

    pub fn close_file(&mut self, fd: usize) -> SystemResult {
        if fd > MAX_OPEN_FILES {
            return SystemResult::MiscellaneousError;
        }
        if self.open_files.len() < fd {
            return SystemResult::FileNotOpen;
        }
        self.open_files.remove(fd);
        SystemResult::Ok
    }

    pub fn read_file(&mut self, fd: usize, len_to_read: usize) -> SystemResult {
        match self.check_file_descriptor(fd) {
            Some(open_file) => {
                if self.get_file(&open_file.file_name).content.len() < len_to_read {
                    return SystemResult::MiscellaneousError;
                }
                if self.check_readability(fd) {
                    let (first, _) = self
                        .get_file(&open_file.file_name)
                        .content
                        .split_at(len_to_read);
                    println!("File content: \"{}\"", first);
                    SystemResult::Ok
                } else {
                    SystemResult::OpenInInvalidMode
                }
            }
            None => SystemResult::FileNotOpen,
        }
    }

    pub fn write_to_file(&mut self, fd: usize, content: String, len_to_write: usize) -> SystemResult {
        let file_name;
        match self.check_file_descriptor(fd) {
            Some(open_file) => {
                if self.check_writability(fd) {
                    file_name = self.get_file(&open_file.file_name).name.to_string();
                } else {
                    return SystemResult::OpenInInvalidMode;
                }
            }
            None => return SystemResult::FileNotOpen,
        }
        let mut len_to_write = len_to_write;
        let content_len = content.chars().count();
        if content_len < len_to_write {
            len_to_write = content_len;
        }
        self.files.get_mut(&file_name).unwrap().content = content[..len_to_write].to_string();
        SystemResult::Ok
    }

    pub fn filename_exists(&self, file_name: &str) -> Option<&_File> {
        for (_, file) in self.files.iter() {
            if file.name == file_name {
                return Some(file);
            }
        }
        None
    }

    pub fn file_is_open(&self, file: &_File) -> Option<(usize, &OpenFile)> {
        for (fd, open_file) in self.open_files.iter().enumerate() {
            if open_file.file_name == file.name {
                return Some((fd, open_file));
            }
        }
        None
    }

    pub fn check_file_descriptor(&self, fd: usize) -> Option<&OpenFile> {
        if self.open_files.len() <= fd {
            return None;
        }

        Some(&self.open_files[fd])
    }

    pub fn check_writability(&self, fd: usize) -> bool {
        (self.open_files[fd].permission as i32 & Permission::Write as i32) != 0
    }

    pub fn check_readability(&self, fd: usize) -> bool {
        (self.open_files[fd].permission as i32 & Permission::Read as i32) != 0
    }

    pub fn check_permission(&self, open_permission: &Permission, file: &_File) -> bool {
        (*open_permission as i32 & file.owner_permission as i32) != 0
    }

    pub fn debug_print(&self) {
        for (_, file) in self.files.iter() {
            println!("{:?}", file);
        }

        for open_file in self.open_files.iter() {
            println!("{:?}", open_file)
        }
    }
}
