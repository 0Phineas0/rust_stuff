/*
 * Made by Guilherme Fontes
*/

use std::sync::{Arc, Mutex};

mod file_system;
mod option_handling;
mod simple_user_input;
mod server_handler;

use file_system::FileSystem;
use server_handler::ClientSystem;

pub static SOCKET_PATH: &'static str = "/tmp/fs_socket";

fn main() {
    let file_system = Arc::new(Mutex::new(FileSystem::new()));

    let client_system = Arc::new(Mutex::new(ClientSystem::new()));

    server_handler::await_connections(SOCKET_PATH, &file_system, &client_system);

    file_system.lock().unwrap().debug_print();
}
