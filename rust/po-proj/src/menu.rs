use crate::simple_user_input::get_input;

enum Bool {
    True(String),
    False(String)
}

enum InputType {
    Numeric(i32),
    Word(String),
    Boolean(Bool)
}

struct Query {
    msg: String,
    input_type: InputType
}

struct OptionForm {
    queries: Vec<Query>
}

pub struct Menu {
    number_of_options: i32,
    label: &'static str,
    options: Vec<&'static str>,
    forms: Vec<OptionForm>
}

pub struct MainMenu {
    menu: Menu
}

pub struct UserMenu {
    menu: Menu
}

pub struct WorksMenu {
    menu: Menu
}

pub struct LoanMenu {
    menu: Menu
}

impl Menu {
    pub fn new(number_of_options: i32, label: String,
    options: Vec<String>) -> Menu{

        Menu { number_of_options, label, options }
    }

    pub fn set_number_of_options(&mut self, _num_options: i32) {
        self.number_of_options = _num_options;
    }

    pub fn get_number_of_options(&self) -> i32 {
        self.number_of_options
    }

    pub fn set_options(&mut self, _options: Vec<String>) {
        self.options = _options;
    }

    pub fn print_menu(&self) {
        println!("───────────────\n> {} <", self.label);
        for (_num, _option) in self.options.iter().enumerate() {
            println!("{} - {}", _num, _option);
        }
    }

    pub fn choose_menu_option(&self) -> i32 {
        loop {
            match get_input("──> ").parse() {
                Ok(n_option) => {
                    let _range = 0..self.number_of_options;
                    if _range.contains(&n_option) {
                        break n_option
                    } else {
                        println!("The option you gave isn't supported!");
                    }
                },
                Err(_) => {
                    println!("Please type a number!");
                    continue;
                }
            }
        }
    }
}

impl MainMenu {
    pub fn new(number_of_options: i32, label: String,
    options: Vec<String>, options_input: Vec<OptionsTypes>) -> Menu {

        let _menu = Menu::new(8, "Main menu", vec!(
                                            "Exit",
                                            "Open file",
                                            "Save file",
                                            "Advance date",
                                            "Show date",
                                            "Go to user menu...",
                                            "Go to works menu...",
                                            "Go to loan menu..."));
    }
}

impl UserMenu {
    pub fn new() -> UserMenu {
        let menu = Menu::new(6, "User menu".to_string(), vec!(
                                            "...Go back to main menu".to_string(),
                                            "Register User".to_string(),
                                            "Show user".to_string(),
                                            "List users".to_string(),
                                            "Show user notifications".to_string(),
                                            "Pay fees".to_string()));

        UserMenu { menu }
    }
}

impl WorksMenu {
    pub fn new() -> WorksMenu {
        let _menu = Menu::new(4, "Works menu".to_string(), vec!(
                                            "...Go back to main menu".to_string(),
                                            "Show work".to_string(),
                                            "List works".to_string(),
                                            "Search".to_string()));
    }
}

impl LoanMenu {
    pub fn new() -> LoanMenu {
        let _menu = Menu::new(3, "Loan Menu".to_string(), vec!(
                                            "...Go back to main menu".to_string(),
                                            "Loan work".to_string(),
                                            "Return work".to_string()));
    }
}
