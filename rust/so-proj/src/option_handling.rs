use crate::file_system::*;

use std::sync::{Arc, Mutex};

pub fn create_file(file_system: &Arc<Mutex<FileSystem>>, args: Vec<String>) -> SystemResult {
    /* get items from args vector: */
    let name = get_string(&args, 0);
    let content = get_string(&args, 1);
    let owner_permission = get_permission(&args, 2);
    let others_permission = get_permission(&args, 3);

    let new_file = _File::new(name, content, owner_permission, others_permission);
    file_system.lock().unwrap().add_file(new_file)
}

pub fn delete_file(file_system: &Arc<Mutex<FileSystem>>, args: Vec<String>) -> SystemResult {
    let name = get_string(&args, 0);

    file_system.lock().unwrap().delete_file(name)
}

pub fn rename_file(file_system: &Arc<Mutex<FileSystem>>, args: Vec<String>) -> SystemResult {
    let name = get_string(&args, 0);
    let new_name = get_string(&args, 1);

    file_system.lock().unwrap().rename_file(name, new_name)
}

pub fn open_file(file_system: &Arc<Mutex<FileSystem>>, args: Vec<String>) -> SystemResult {
    let name = get_string(&args, 0);
    let open_permission = get_permission(&args, 1);

    file_system.lock().unwrap().open_file(name, open_permission)
}

pub fn close_file(file_system: &Arc<Mutex<FileSystem>>, args: Vec<String>) -> SystemResult {
    let fd = get_usize(&args, 0);

    file_system.lock().unwrap().close_file(fd)
}

pub fn read_file(file_system: &Arc<Mutex<FileSystem>>, args: Vec<String>) -> SystemResult {
    let fd = get_usize(&args, 0);
    let len_to_read = get_usize(&args, 1);

    file_system.lock().unwrap().read_file(fd, len_to_read)
}

pub fn write_to_file(file_system: &Arc<Mutex<FileSystem>>, args: Vec<String>) -> SystemResult {
    let fd = get_usize(&args, 0);
    let content = get_string(&args, 1);
    let len_to_write = get_usize(&args, 2);

    file_system
        .lock()
        .unwrap()
        .write_to_file(fd, content, len_to_write)
}

fn get_usize(args: &Vec<String>, index: usize) -> usize {
    match args.get(index) {
        Some(_goes_into_name) => match _goes_into_name.trim().parse() {
            Ok(_gets_returned) => _gets_returned,
            Err(e) => panic!(
                "args vector should have a number at index {index}!: {error}",
                index = index,
                error = e
            ),
        },
        None => panic!("args vector should have name at index {}!", index),
    }
}

fn get_string(args: &Vec<String>, index: usize) -> String {
    match args.get(index) {
        Some(_goes_into_name) => _goes_into_name.to_string(),
        None => panic!("args vector should have name at index {}!", index),
    }
}

fn get_permission(args: &Vec<String>, index: usize) -> Permission {
    match args.get(index) {
        Some(element) => {
            match element.as_str() {
                "Write" => Permission::Write,
                "Read" => Permission::Read,
                "ReadWrite" => Permission::ReadWrite,
                "None" => Permission::None,
                _ => panic!("FSReturn not compatible!"),
            }
        }
        None => panic!("args vector should have name at index {}!", index),
    }
}
