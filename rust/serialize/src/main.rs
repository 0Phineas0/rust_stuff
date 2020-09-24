use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::HashMap;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread;

const DATA_FILE: &'static str = "data/Data.json";
const CONTACTS_LIST_FILE: &'static str = "data/Contacts_list.json";

#[derive(Debug, Serialize, Deserialize)]
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

    fn new_err(query: Query) -> Self {
        Feedback { result: Err(query) }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    name: String,
    phone: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contacts {
    contacts_list: HashMap<String, HashMap<u64, Contact>>, // <account name, contacts list>
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    clients: Vec<Account>,
}

impl Data {
    fn new() -> Self {
        Data {
            clients: Vec::new(),
        }
    }

    fn check_login(&self, creds: &Account) -> Option<Account> {
        self.clients.iter().find_map(|acc| {
            if acc.name == creds.name && acc.password == creds.password {
                Some(Account::new(&acc.name, &acc.password))
            } else {
                None
            }
        })
    }

    // fn search_client(&self, creds: &Account) -> Option<&Account> {
    //     self.clients.iter().find_map(|acc| {
    //         if acc.name == creds.name {
    //             Some(acc)
    //         } else {
    //             None
    //         }
    //     })
    // }

    fn add_client(&mut self, name: &str, password: &str) -> bool {
        if let Some(_) =
            self.clients
                .iter()
                .find_map(|acc| if acc.name == name { Some(acc) } else { None })
        {
            return false;
        }

        let (name, password) = (name.to_owned(), password.to_owned());
        self.clients.push(Account { name, password });

        true
    }

    fn save(&self) {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(DATA_FILE)
        {
            Ok(_file) => _file,
            Err(_error) => panic!("Error trying to open contacts list file!"),
        };
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn recover(&mut self) {
        let file = match OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(DATA_FILE)
        {
            Ok(_file) => _file,
            Err(_error) => panic!("Error trying to open contacts list file!"),
        };
        if file.metadata().unwrap().len() > 0 {
            *self = serde_json::from_reader(file).unwrap()
        }
    }
}

impl Contact {
    fn new(name: &str, phone: u64) -> Self {
        let name = name.to_owned();
        Contact { name, phone }
    }
}

impl Account {
    fn new(name: &str, password: &str) -> Self {
        let name = name.to_owned();
        let password = password.to_owned();
        Account { name, password }
    }
}

impl Contacts {
    fn new() -> Self {
        Contacts {
            contacts_list: HashMap::default(),
        }
    }

    fn save(&self) {
        let serialized = serde_json::to_string(&self).unwrap();
        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(CONTACTS_LIST_FILE)
        {
            Ok(_file) => _file,
            Err(_error) => panic!("Error trying to open contacts list file!"),
        };
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn recover(&mut self) {
        let file = match OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(CONTACTS_LIST_FILE)
        {
            Ok(_file) => _file,
            Err(_error) => panic!("Error trying to open contacts list file!"),
        };
        if file.metadata().unwrap().len() > 0 {
            *self = serde_json::from_reader(file).unwrap()
        }
    }

    fn add_contact(&mut self, stream: &mut TcpStream, account: &str, name: &str, phone: u64) {
        if self
            .contacts_list
            .get(account).unwrap()
            .contains_key(&phone)
        {
            serde_json::to_writer(stream, &Feedback::new_err(Query::Add)).unwrap();
        } else {
            let new_contact = Contact::new(name, phone);
            self.contacts_list
                .get_mut(account)
                .unwrap()
                .insert(phone, new_contact);
            serde_json::to_writer(stream, &Feedback::new_ok(Query::Add, None, None)).unwrap();
        }
    }

    fn remove(&mut self, stream: &mut TcpStream, account: &str, phone: u64) {
        match self.contacts_list.get_mut(account).unwrap().remove(&phone) {
            Some(_contact) => serde_json::to_writer(
                stream,
                &Feedback::new_ok(Query::Remove, Some(_contact.name), Some(_contact.phone)),
            )
            .unwrap(),
            None => serde_json::to_writer(stream, &Feedback::new_err(Query::Remove)).unwrap(),
        }
    }

    fn search_by_name(&self, mut stream: &mut TcpStream, account: &str, name: &str) {
        let search: Vec<u64> = self
            .contacts_list
            .get(account)
            .unwrap()
            .iter()
            .filter_map(|(&phone, contact)| {
                if contact.name == name {
                    Some(phone)
                } else {
                    None
                }
            })
            .collect();
        if search.is_empty() {
            serde_json::to_writer(
                stream,
                &Feedback {
                    result: Err(Query::SearchByName),
                },
            )
            .unwrap();
            return;
        }
        for phone in search.iter() {
            serde_json::to_writer(
                &mut stream,
                &Feedback::new_ok(Query::SearchByName, None, Some(phone.to_owned())),
            )
            .unwrap();
        }
        serde_json::to_writer(stream, &Feedback::new_ok(Query::Done, None, None)).unwrap()
    }

    fn search_by_number(&self, stream: &mut TcpStream, account: &str, phone: u64) {
        let opt = self.contacts_list.get(account).unwrap().get(&phone);
        if let Some(contact) = opt {
            serde_json::to_writer(
                stream,
                &Feedback::new_ok(Query::SearchByPhone, Some(contact.name.clone()), None),
            )
            .unwrap()
        } else {
            serde_json::to_writer(stream, &Feedback::new_err(Query::SearchByPhone)).unwrap()
        }
    }

    fn show_list(&self, mut stream: &mut TcpStream, account: &str) {
        println!("<name>: <phone number>");
        for contact in self.contacts_list.get(account).unwrap().iter() {
            println!("{:?}", contact);
            serde_json::to_writer(
                &mut stream,
                &Feedback::new_ok(
                    Query::ShowList,
                    Some(contact.1.name.clone()),
                    Some(*contact.0),
                ),
            )
            .unwrap();
        }
    }
}

fn handle_client(mut stream: TcpStream, data: Arc<RwLock<Data>>, contacts: Arc<RwLock<Contacts>>) {
    let reader = stream.try_clone().unwrap();
    let mut reader = std::io::BufReader::new(reader);

    let account_name = loop {
        let mut de = serde_json::Deserializer::from_reader(&mut reader);
        let ans: Ans = match Feedback::deserialize(&mut de) {
            Ok(_feedback) => _feedback.result.ok().unwrap(),
            Err(_) => unreachable!(),
        };

        let mut de = serde_json::Deserializer::from_reader(&mut reader);
        let creds: Account = match Account::deserialize(&mut de) {
            Ok(_creds) => _creds,
            Err(_) => unreachable!(),
        };

        match ans.query {
            Query::Login => {
                let account = data.read().unwrap().check_login(&creds);
                if let Some(acc) = account {
                    serde_json::to_writer(&mut stream, &Feedback::new_ok(Query::Login, None, None))
                        .unwrap();

                    break acc.name;
                } else {
                    serde_json::to_writer(&mut stream, &Feedback::new_err(Query::Login)).unwrap();
                }
            }
            Query::CreateAccount => {
                if data
                    .write()
                    .unwrap()
                    .add_client(&creds.name, &creds.password)
                {
                    serde_json::to_writer(
                        &mut stream,
                        &Feedback::new_ok(Query::CreateAccount, None, None),
                    )
                    .unwrap();

                    break creds.name.to_string();
                } else {
                    serde_json::to_writer(&mut stream, &Feedback::new_err(Query::CreateAccount))
                        .unwrap();
                }
            }
            _ => unreachable!(),
        }
    };
    contacts.write().unwrap().contacts_list.entry(account_name.clone()).or_insert(HashMap::default());
    loop {
        let mut de = serde_json::Deserializer::from_reader(&mut reader);
        let ans: Ans = match Feedback::deserialize(&mut de) {
            Ok(_feedback) => _feedback.result.ok().unwrap(),
            Err(_) => break,
        };
        println!("{:?}", ans.query);
        match ans.query {
            Query::Save => {
                data.read().unwrap().save();
                contacts.read().unwrap().save();

                serde_json::to_writer(&mut stream, &Feedback::new_ok(Query::Done, None, None))
                    .unwrap()
            }
            Query::Add => contacts.write().unwrap().add_contact(
                &mut stream,
                &account_name,
                &ans.name.unwrap(),
                ans.phone.unwrap(),
            ),
            Query::Remove => {
                contacts
                    .write()
                    .unwrap()
                    .remove(&mut stream, &account_name, ans.phone.unwrap())
            }
            Query::SearchByName => {
                contacts
                    .read()
                    .unwrap()
                    .search_by_name(&mut stream, &account_name, &ans.name.unwrap())
            }
            Query::SearchByPhone => {
                contacts
                    .read()
                    .unwrap()
                    .search_by_number(&mut stream, &account_name, ans.phone.unwrap())
            }
            Query::ShowList => {
                contacts.read().unwrap().show_list(&mut stream, &account_name);

                serde_json::to_writer(&mut stream, &Feedback::new_ok(Query::Done, None, None))
                    .unwrap()
            }
            _ => {}
        }
    }
}

fn main() {
    let data = Arc::new(RwLock::new(Data::new()));
    let contacts = Arc::new(RwLock::new(Contacts::new()));
    data.write().unwrap().recover();
    contacts.write().unwrap().recover();

    let listener = TcpListener::bind("3.17.149.107:8080").unwrap();
    let mut client_count = 1;
    for stream in listener.incoming() {
        let data_clone = Arc::clone(&data);
        let contacts_clone = Arc::clone(&contacts);
        thread::Builder::new()
            .name(format!("client {}", client_count))
            .spawn(|| {
                handle_client(stream.unwrap(), data_clone, contacts_clone);
            })
            .unwrap();
        client_count += 1;
    }
}
