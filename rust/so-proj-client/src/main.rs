mod simple_user_input;
mod client_input_handling;

extern crate rpassword;
use num_enum::TryFromPrimitive;

use client_input_handling::*;
use simple_user_input::get_input;
use std::convert::TryFrom;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::os::unix::net::UnixStream;
use std::net::Shutdown;
use std::{thread, time};

#[derive(Eq, PartialEq, TryFromPrimitive)]
#[repr(i32)]
enum SystemResult {
    Ok = 0,
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
    ClosedConnection = -30,
}

struct Menu {
    label: &'static str,
    options_input: Vec<&'static str>,
    options: Vec<&'static str>,
}

impl Menu {
    fn new(
        label: &'static str,
        options_input: Vec<&'static str>,
        options: Vec<&'static str>,
    ) -> Self {
        Menu {
            label,
            options_input,
            options,
        }
    }

    pub fn print_menu(&self) {
        println!("\n─────────────────────");
        println!("{}", self.label);
        println!("─────────────────────");
        for (inp, option) in self.options_input.iter().zip(self.options.iter()) {
            println!("{} - {}", inp, option);
        }
        println!("─────────────────────\n");
    }
}

fn create_main_menu() -> Menu {
    Menu::new(
        "Main menu",
        vec!["e", "r", "l"],
        vec!["...Exit", "Register", "Login"],
    )
}

fn create_client_menu() -> Menu {
    Menu::new(
        "Client Menu",
        vec!["e", "c", "d", "r", "o", "x", "l", "w"],
        vec![
            "...Exit (Logout)",
            "Create file",
            "Delete file",
            "Rename file",
            "Open file",
            "Close file",
            "Read file",
            "Write to file",
        ],
    )
}

fn print_result(result: SystemResult) {
    let report = match result {
        SystemResult::Ok => return,
        SystemResult::FileAlreadyExists => "File already exists!",
        SystemResult::FileDoesntExist => "File doesn't exist!",
        SystemResult::FileNotOpen => "File isn't open!",
        SystemResult::FileAlreadyOpen => "File already open!",
        SystemResult::OpenInInvalidMode => "File can't be read!",
        SystemResult::PermissionDenied => "Permission denied!",
        SystemResult::ReachedMaxOpenFiles => "Can't open more files!",
        SystemResult::MiscellaneousError => "Other error!",
        SystemResult::ClientAlreadyExists => "Client already exists! Try another name!",
        SystemResult::ClientDoesntExist => "Client doesn't exist! Maybe you got the name wrong!?",
        SystemResult::WrongCredentials => "Wrong credentials!",
        SystemResult::ClosedConnection => "Connection closed, idkw!?",
    };

    println!("{}", report);
}

fn register_client(stream: &mut UnixStream) {
    let new_name = get_input("New name ─> ");
    let new_password = match rpassword::read_password_from_tty(Some("New password ─> ")) {
        Ok(_goes_into_password) => _goes_into_password,
        Err(e) => panic!(e),
    };

    write_str(
        stream,
        &format!(
            "{menu}|{option}|{name}|{password}\n",
            menu = "m",
            option = "r",
            name = new_name,
            password = new_password
        ),
    )
}

fn login_client(stream: &mut UnixStream) {
    let name = get_input("Name ─> ");
    let password = match rpassword::read_password_from_tty(Some("Password ─> ")) {
        Ok(_goes_into_password) => _goes_into_password,
        Err(e) => panic!(e),
    };
    write_str(
        stream,
        &format!(
            "{menu}|{option}|{name}|{password}\n",
            menu = "m",
            option = "l",
            name = name,
            password = password
        ),
    )
}

fn write_str(stream: &mut UnixStream, to_write: &str) {
    match stream.write_all(to_write.as_bytes()) {
        Ok(_) => { /* Successful write to socket! */ }
        Err(e) => panic!(e),
    }
}

fn get_result(line: String) -> SystemResult {
    let num: i32 = match line.trim().parse() {
        Ok(_goes_into_num) => _goes_into_num,
        Err(e) => panic!("Wasn't able to parse string to i32!: {}", e),
    };
    match SystemResult::try_from(num) {
        Ok(_fs_result_is_returned) => _fs_result_is_returned,
        Err(e) => panic!("SystemResult not compatible!: {}", e),
    }
}

fn receive_result(stream: UnixStream) -> SystemResult {
    let buf_stream = BufReader::new(stream);
    for line in buf_stream.lines() {
        return get_result(line.unwrap());
    }
    SystemResult::ClosedConnection
}

static SOCKET_PATH: &'static str = "/tmp/fs_socket";

fn main() {
    let mut stream = match UnixStream::connect(SOCKET_PATH) {
        Ok(_goes_to_stream) => _goes_to_stream,
        Err(e) => panic!(e),
    };

    let main_menu = create_main_menu();
    let client_menu = create_client_menu();

    loop {
        main_menu.print_menu();

        match get_input("──> ").as_str() {
            "e" | "E" => break,
            "r" | "R" => register_client(&mut stream),
            "l" | "L" => login_client(&mut stream),
            _ => {
                println!("\nType one of the supported options below!");
                thread::sleep(time::Duration::new(1, 100_000_000));
                continue;
            }
        };

        let result = receive_result(match stream.try_clone() {
            Ok(_cloned_stream) => _cloned_stream,
            Err(e) => panic!("Couldn't clone stream!: {}", e),
        });

        match result {
            SystemResult::ClosedConnection => {
                eprintln!("Connection forcibly closed!");
                return;
            }
            SystemResult::Ok => {
                loop {
                    client_menu.print_menu();

                    let to_write: String = match get_input("──> ").as_str() {
                        "e" | "E" => break,
                        "c" | "C" => create_file(),
                        "d" | "D" => delete_file(),
                        "r" | "R" => rename_file(),
                        "o" | "O" => open_file(),
                        "x" | "X" => close_file(),
                        "l" | "L" => read_file(),
                        "w" | "W" => write_to_file(),
                        _ => {
                            println!("\nType one of the supported options below!");
                            thread::sleep(time::Duration::new(1, 100_000_000));
                            continue;
                        }
                    };
                    write_str(&mut stream, &to_write);
                    let result = receive_result(match stream.try_clone() {
                        Ok(_cloned_stream) => _cloned_stream,
                        Err(e) => panic!("Couldn't clone stream!: {}", e),
                    });
                    print_result(result)
                }
            }
            _ => print_result(result),
        }
    }
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => { /* UnixStream shutdown Successful! */ },
        Err(e) => panic!("UnixStream shutdown failed!: {error}", error = e)
    }
}
