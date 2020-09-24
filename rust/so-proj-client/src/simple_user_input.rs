use std::io::{self, Write};
use std::{thread, time};

#[derive(Debug)]
pub enum Permission {
    None = 0,      // 0b0000
    Write = 1,     // 0b0001
    Read = 2,      // 0b0010
    ReadWrite = 3, // 0b0011
}

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    match io::stdout().flush() {
        Ok(_flush_successful) => {},
        Err(e) => println!("Error on stdout flush: {}", e),
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(e) => println!("Error on read line: {}", e),
    }
    input.trim().to_string()
}

pub fn read_filename_input(form: &str) -> String {
    get_input(form)
}

pub fn read_file_content_input(form: &str) -> String {
    get_input(form)
}

pub fn read_permission_input_aux(form: &str, option: bool) -> Permission {
    loop {
        match get_input(form).as_str() {
            "R" | "r" => break Permission::Read,
            "W" | "w" => break Permission::Write,
            "RW" | "rw" | "Rw" | "rW" => break Permission::ReadWrite,
            "N" | "n" if option => break Permission::None,
            _ => {
                println!("Please input one of the given permission types!");
                thread::sleep(time::Duration::new(1, 100_000_000));
                continue;
            }
        }
    }
}

pub fn read_permission_input(form: &str) -> Permission {
    read_permission_input_aux(form, true)
}

pub fn read_open_permission_input(form: &str) -> Permission {
    read_permission_input_aux(form, false)
}

pub fn read_file_descriptor_input(form: &str) -> usize {
    read_unsigned_number_input(form)
}

pub fn read_unsigned_number_input(form: &str) -> usize {
    loop {
        match get_input(form).trim().parse::<i32>() {
            Ok(fd) => {
                if fd < 0 {
                    println!(
                        "{} is lower than 0!\nPlease type a number greater than or equal to 0!",
                        fd
                    );
                    thread::sleep(time::Duration::new(1, 100_000_000));
                    continue;
                }
                return fd as usize;
            }
            Err(_) => {
                println!("You need to type number greater than or equal to 0!");
                thread::sleep(time::Duration::new(1, 100_000_000));
                continue;
            }
        }
    }
}
