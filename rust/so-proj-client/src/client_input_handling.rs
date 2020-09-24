use crate::simple_user_input::*;

pub fn create_file() -> String {
    let name = read_filename_input("Filename ─> ");
    let content = read_file_content_input("File content ─> ");
    let owner_permission = read_permission_input("Owner permission [ R | W | RW | N ] ─> ");
    let others_permission = read_permission_input("Others permission [ R | W | RW | N ] ─> ");

    format!(
        "{menu}|{option}|{name}|{content}|{owner_permission:?}|{others_permission:?}\n",
        menu = "c",
        option = "c",
        name = name,
        content = content,
        owner_permission = owner_permission,
        others_permission = others_permission
    )
}

pub fn delete_file() -> String {
    let name = read_filename_input("Filename -> ");

    format!(
        "{menu}|{option}|{name}\n",
        menu = "c",
        option = "d",
        name = name
    )
}

pub fn rename_file() -> String {
    let name = read_filename_input("Filename ─> ");
    let new_name = read_filename_input("New filename ─> ");

    format!(
        "{menu}|{option}|{name}|{new_name}\n",
        menu = "c",
        option = "r",
        name = name,
        new_name = new_name
    )
}

pub fn open_file() -> String {
    let name = read_filename_input("Filename ─> ");
    let open_permission = read_open_permission_input("Open permission [ R | W | RW ] ─> ");

    format!(
        "{menu}|{option}|{name}|{open_permission:?}\n",
        menu = "c",
        option = "o",
        name = name,
        open_permission = open_permission
    )
}

pub fn close_file() -> String {
    let fd = read_unsigned_number_input("File descriptor ─> ");

    format!("{menu}|{option}|{fd}", menu = "c", option = "x", fd = fd)
}

pub fn read_file() -> String {
    let fd = read_file_descriptor_input("File descriptor ─> ");
    let len_to_read = read_unsigned_number_input("Length to read ─> ");

    format!(
        "{menu}|{option}|{fd}|{len_to_read}\n",
        menu = "c",
        option = "l",
        fd = fd,
        len_to_read = len_to_read
    )
}

pub fn write_to_file() -> String {
    let fd = read_file_descriptor_input("File descriptor ─> ");
    let content = read_file_content_input("File content ─> ");
    let len_to_write = read_unsigned_number_input("Length to write ─> ");

    format!(
        "{menu}|{option}|{fd}|{content}|{len_to_write}\n",
        menu = "c",
        option = "w",
        fd = fd,
        content = content,
        len_to_write = len_to_write
    )
}
