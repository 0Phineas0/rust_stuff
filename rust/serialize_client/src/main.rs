extern crate rpassword;

use serde::{Deserialize, Serialize};
use serde_json;

mod simple_user_input;
use simple_user_input::get_input;

use std::io::BufReader;
use std::net::TcpStream;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
enum Query {
    Save,
    Add,
    Remove,
    SearchByName,
    SearchByPhone,
    ShowList,
    Done,
    Login,
    CreateAccount,
}

#[derive(Serialize, Deserialize)]
struct Ans {
    query: Query,
    name: Option<String>,
    phone: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct Feedback {
    result: Result<Ans, Query>,
}

impl Feedback {
    fn new_ok(query: Query, name: Option<String>, phone: Option<u64>) -> Self {
        Feedback {
            result: Ok(Ans { query, name, phone }),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    name: String,
    password: String,
}

impl Account {
    fn new(name: &str, password: &str) -> Self {
        let name = name.to_string();
        let password = password.to_string();

        Account { name, password }
    }
}

fn login(mut stream: &mut TcpStream, mut reader: &mut BufReader<TcpStream>) -> bool {
    loop {
        let name = get_input("name: ");
        let pass = rpassword::prompt_password_stdout("password: ").unwrap();

        serde_json::to_writer(&mut stream, &Feedback::new_ok(Query::Login, None, None)).unwrap();
        serde_json::to_writer(&mut stream, &Account::new(&name, &pass)).unwrap();

        let mut de = serde_json::Deserializer::from_reader(&mut reader);
        if let Ok(_) = Feedback::deserialize(&mut de).unwrap().result {
            println!("Logged in successfully");
            break true;
        } else {
            println!("Credentials are incorrect!");
            println!("0 - Exit\n1 - Try to login again");
            match get_input("──> ").as_str() {
                "0" => break false,
                "1" => continue,
                _ => unreachable!(),
            }
        }
    }
}

fn register(mut stream: &mut TcpStream, mut reader: &mut BufReader<TcpStream>) -> bool {
    loop {
        let name = get_input("name: ");
        let pass = rpassword::prompt_password_stdout("password: ").unwrap();

        serde_json::to_writer(
            &mut stream,
            &Feedback::new_ok(Query::CreateAccount, None, None),
        )
        .unwrap();
        serde_json::to_writer(&mut stream, &Account::new(&name, &pass)).unwrap();

        let mut de = serde_json::Deserializer::from_reader(&mut reader);
        if let Ok(_) = Feedback::deserialize(&mut de).unwrap().result {
            println!("Account created successfully");
            break true;
        } else {
            println!("Account with name {} already exists!", name);
            println!("0 - Exit\n1 - Try to register again");
            match get_input("──> ").as_str() {
                "0" => break false,
                "1" => continue,
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let mut stream = TcpStream::connect("3.17.149.107:54321").unwrap();
    let reader = stream.try_clone().unwrap();
    let mut reader = BufReader::new(reader);

    loop {
        println!("");
        println!("0 - Exit\n1 - Login\n2 - Create account",);

        let input = get_input("──> ");
        println!("");
        match input.as_str() {
            "0" => return,
            "1" => {
                if login(&mut stream, &mut reader) {
                    break;
                }
            }
            "2" => {
                if register(&mut stream, &mut reader) {
                    break;
                }
            }
            _ => {
                println!("Option not supported!");
                continue;
            }
        }
    }

    loop {
        println!("");
        println!(
            "0 - Exit\n1 - Add contact\n2 - Remove contact\n3 - Search contact\n4 - Show contacts"
        );
        let input = get_input("──> ");
        println!("");
        match input.as_str() {
            "0" => {
                serde_json::to_writer(&mut stream, &Feedback::new_ok(Query::Save, None, None))
                    .unwrap();

                let mut de = serde_json::Deserializer::from_reader(&mut reader);
                let ans: Ans = Feedback::deserialize(&mut de).unwrap().result.ok().unwrap();

                if ans.query == Query::Done {
                    break;
                } else {
                    unreachable!()
                }
            }
            "1" => {
                let name = get_input("name: ");
                let phone: u64 = loop {
                    match get_input("phone: ").parse() {
                        Ok(_phone) => break _phone,
                        Err(_) => continue,
                    }
                };
                serde_json::to_writer(
                    &mut stream,
                    &Feedback::new_ok(Query::Add, Some(name.clone()), Some(phone)),
                )
                .unwrap();

                let mut de = serde_json::Deserializer::from_reader(&mut reader);
                let feedback = Feedback::deserialize(&mut de).unwrap();
                if let Ok(_) = feedback.result {
                    println!("Added contact!");
                } else {
                    println!("There is already a contact with name \"{}\"", name);
                }
            }
            "2" => {
                let phone: u64 = loop {
                    match get_input("phone: ").parse() {
                        Ok(_phone) => break _phone,
                        Err(_) => continue,
                    }
                };
                serde_json::to_writer(
                    &mut stream,
                    &Feedback::new_ok(Query::Remove, None, Some(phone)),
                )
                .unwrap();

                let mut de = serde_json::Deserializer::from_reader(&mut reader);
                let feedback = Feedback::deserialize(&mut de).unwrap();
                if let Ok(_) = feedback.result {
                    println!(
                        "Contact with phone number {} was removed successfully!",
                        phone
                    );
                } else {
                    println!("Didn't find contact with phone number \"{}\"", phone);
                }
            }
            "3" => loop {
                println!("0 - Back\n1 - Search by name\n2 - Search by number");
                let search_option = get_input("──> ");
                println!("");
                match search_option.as_str() {
                    "0" => break,
                    "1" => {
                        let name = get_input("name: ");
                        serde_json::to_writer(
                            &mut stream,
                            &Feedback::new_ok(Query::SearchByName, Some(name.clone()), None),
                        )
                        .unwrap();

                        let mut de = serde_json::Deserializer::from_reader(&mut reader);
                        let feedback = Feedback::deserialize(&mut de).unwrap();
                        if let Ok(ans) = feedback.result {
                            println!("Found contact with phone number {}", ans.phone.unwrap());
                        } else {
                            println!("Didn't find any contact with the name \"{}\"", name);
                        }
                    }
                    "2" => {
                        let phone: u64 = loop {
                            match get_input("phone: ").parse() {
                                Ok(_phone) => break _phone,
                                Err(_) => continue,
                            }
                        };
                        serde_json::to_writer(
                            &mut stream,
                            &Feedback::new_ok(Query::SearchByPhone, None, Some(phone)),
                        )
                        .unwrap();

                        let mut de = serde_json::Deserializer::from_reader(&mut reader);
                        let feedback = Feedback::deserialize(&mut de).unwrap();
                        if let Ok(ans) = feedback.result {
                            println!("Found contact with name \"{}\"", ans.name.unwrap());
                        } else {
                            println!("Didn't find contact with phone number {}", phone);
                        }
                    }
                    _ => continue,
                }
            },
            "4" => {
                serde_json::to_writer(&mut stream, &Feedback::new_ok(Query::ShowList, None, None))
                    .unwrap();

                println!("<name>: <phone number>");
                loop {
                    let mut de = serde_json::Deserializer::from_reader(&mut reader);
                    let ans: Ans = Feedback::deserialize(&mut de).unwrap().result.ok().unwrap();
                    match ans.query {
                        Query::ShowList => {
                            println!("{}: {}", ans.name.unwrap(), ans.phone.unwrap())
                        }
                        Query::Done => break,
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
