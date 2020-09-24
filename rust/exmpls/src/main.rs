enum Stuff {
    Stuffy,
    Enh
}

fn main() {
    let something = Stuff::Stuffy;

    match something {
        Stuff::Stuffy => println!("It's a stuffy!!"),
        Stuff::Enh => println!("It's a enh :(")
    }
}
