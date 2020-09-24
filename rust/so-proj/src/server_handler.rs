extern crate bincode;
extern crate rustc_serialize;

use crate::file_system::*;
use crate::option_handling::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::io::{BufReader, BufWriter};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Serialize, Deserialize, RustcEncodable, RustcDecodable)]
pub struct Client {
    name: String,
    password: String,
}
#[derive(Debug, Serialize, Deserialize, RustcEncodable, RustcDecodable)]
pub struct ClientSystem {
    clients: HashMap<String, Client>,
    clients_online: Vec<String>,
}

impl Client {
    fn new(name: String, password: String) -> Self {
        Client { name, password }
    }
}

impl ClientSystem {
    pub fn new() -> Self {
        ClientSystem {
            clients: HashMap::default(),
            clients_online: Vec::default(),
        }
    }

    pub fn save_status(&self) {
        let mut writer = BufWriter::new(File::create("ClientList.bin").unwrap());
        bincode::serialize_into(&mut writer, self).unwrap();
    }

    pub fn recover_status(&mut self) {
        let mut reader = BufReader::new(File::open("ClientList.bin").unwrap());
        let decoded: ClientSystem = bincode::deserialize_from(&mut reader).unwrap();
        *self = decoded;
    }

    pub fn register_client(&mut self, name: String, password: String) -> SystemResult {
        match self.client_exists(&name) {
            Some(_) => SystemResult::ClientDoesntExist,
            None => {
                let client = Client::new(name.clone(), password);
                self.clients.insert(name, client);
                SystemResult::Ok
            }
        }
    }

    pub fn login_client(&mut self, name: String, password: String) -> SystemResult {
        match self.client_exists(&name) {
            Some(client) => {
                if client.password == password {
                    self.clients_online.push(name);
                    SystemResult::Ok
                } else {
                    SystemResult::WrongCredentials
                }
            }
            None => SystemResult::ClientDoesntExist,
        }
    }

    pub fn client_exists(&self, name: &str) -> Option<&Client> {
        for (_, client) in self.clients.iter() {
            if client.name == name {
                return Some(&client);
            }
        }
        None
    }
}

pub fn unlink(socket_path: &str) {
    match std::fs::remove_file(socket_path) {
        Ok(_) => { /* Removed socket file successfully */ }
        Err(_) => { /* No socket file */ }
    }
}

pub fn bind(socket_path: &str) -> UnixListener {
    match UnixListener::bind(socket_path) {
        Ok(_goes_to_listener) => _goes_to_listener,
        Err(e) => panic!("Not able to bind to socket path: {}", e),
    }
}

fn send_result(stream: &mut UnixStream, bytes: &[u8]) {
    match stream.write_all(bytes) {
        Ok(_) => { /* Successful write to socket! */ }
        Err(e) => panic!("Not able to write to socket!: {}", e),
    };
}

pub fn client_handler(
    mut stream: UnixStream,
    file_system: Arc<Mutex<FileSystem>>,
    client_system: Arc<Mutex<ClientSystem>>,
) {
    let buf_stream = BufReader::new(match stream.try_clone() {
        Ok(_cloned_stream) => _cloned_stream,
        Err(e) => panic!("Not able to clone UnixStream!: {}", e),
    });
    let temp_stream = BufReader::new(match stream.try_clone() {
        Ok(_cloned_stream) => _cloned_stream,
        Err(e) => panic!("Not able to clone UnixStream!: {}", e),
    });
    let mut sub_strings: Vec<String>;
    let name: String;
    for line in temp_stream.lines() {
        sub_strings = line.unwrap().split("|").map(|s| s.to_string()).collect();
        name = match sub_strings.get(2) {
            Some(_goes_into_name) => _goes_into_name.to_string(),
            None => panic!("There should be a string at index 2"),
        };
        break;
    }

    for line in buf_stream.lines() {
        sub_strings = line.unwrap().split("|").map(|s| s.to_string()).collect();
        let system_result = interact(sub_strings, &file_system, &client_system);
        send_result(
            &mut stream,
            format!("{}\n", system_result as i32).as_bytes(),
        );
        print_debug(&file_system, &client_system)
    }
}

pub fn interact(
    args: Vec<String>,
    file_system: &Arc<Mutex<FileSystem>>,
    client_system: &Arc<Mutex<ClientSystem>>,
) -> SystemResult {
    let system_result: SystemResult;
    match args.get(0) {
        Some(arg) => {
            let option = args[1].as_str();
            match arg.as_str() {
                "c" => {
                    system_result = interact_client_menu(file_system, option, args[2..].to_vec());
                }
                "m" => {
                    system_result = interact_main_menu(client_system, option, args[2..].to_vec());
                }
                _ => panic!("This menu tag shouldn't exist!"),
            }
        }
        None => panic!("Arguments vector is empty!"),
    }
    system_result
}

pub fn interact_main_menu(
    client_system: &Arc<Mutex<ClientSystem>>,
    option: &str,
    args: Vec<String>,
) -> SystemResult {
    match option {
        "r" | "R" => client_system.lock().unwrap().register_client(
            args.get(0)
                .expect("args vector should have name at index 0")
                .clone(),
            args.get(1)
                .expect("args vector should have name at index 1")
                .clone(),
        ),
        "l" | "L" => client_system.lock().unwrap().login_client(
            args.get(0)
                .expect("args vector should have name at index 0")
                .clone(),
            args.get(1)
                .expect("args vector should have name at index 1")
                .clone(),
        ),
        "e" | "E" => SystemResult::Exit,
        other_option => {
            /* Option shouldn't be reached */
            panic!("There shouldn't be this option!: {}", other_option)
        }
    }
}

pub fn interact_client_menu(
    file_system: &Arc<Mutex<FileSystem>>,
    option: &str,
    args: Vec<String>,
) -> SystemResult {
    match option {
        "e" | "E" => SystemResult::Exit,
        "c" | "C" => create_file(file_system, args),
        "d" | "D" => delete_file(file_system, args),
        "r" | "R" => rename_file(file_system, args),
        "o" | "O" => open_file(file_system, args),
        "x" | "X" => close_file(file_system, args),
        "l" | "L" => read_file(file_system, args),
        "w" | "W" => write_to_file(file_system, args),
        other_option => {
            /* Option shouldn't be reached */
            panic!("There shouldn't be this option!: {}", other_option)
        }
    }
}

pub fn await_connections(
    socket_path: &str,
    file_system: &Arc<Mutex<FileSystem>>,
    client_system: &Arc<Mutex<ClientSystem>>,
) {
    unlink(socket_path);
    let listener = bind(socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let file_system_ref = Arc::clone(file_system);
                let client_system_ref = Arc::clone(client_system);
                thread::spawn(|| client_handler(stream, file_system_ref, client_system_ref));
            }
            Err(e) => {
                eprintln!("{:?}", e);
                break;
            }
        }
    }
}

fn print_debug(file_system: &Arc<Mutex<FileSystem>>, client_system: &Arc<Mutex<ClientSystem>>) {
    println!("FileSystem:\n{:?}\n", file_system.lock().unwrap());
    println!("ClientSystem:\n{:?}", client_system.lock().unwrap())
}
