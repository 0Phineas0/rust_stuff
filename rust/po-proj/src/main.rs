// mod simple_user_input;
mod simple_user_input;
mod menu;
use menu::Menu;

fn main() {
    show_menu();
}

fn show_menu() {
    // Main menu
    let _main_menu = MainMenu::new();
    // User menu
    let _user_menu = UserMenu::new( );
    // Works menu
    let _works_menu = WorksMenu::new();
    // Loan menu
    let _loan_menu = LoanMenu::new();

    _main_menu.print_menu(); // Print main menu
    let mut _option = _main_menu.choose_menu_option();

    _user_menu.print_menu();
    _user_menu.choose_menu_option();

    // loop {
    //     match option {
    //
    //     }
    // }
}
