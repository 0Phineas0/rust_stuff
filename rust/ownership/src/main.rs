fn main() {
    let mut a = [String::from("Hello"), String::from(", "), String::from("World")];

    let some_string = func(&mut a);

    println!("{}", some_string);

    println!("{:?}", a);
}

fn func(arg: &mut [String; 3]) -> &str{
    arg[1] = String::from(" Shitty ");
    &arg[1]
}
