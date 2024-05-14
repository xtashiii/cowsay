
use std::io;
fn main()
{
    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Error");

    let dash = "_".repeat(2 + text.chars().count() - 1);
    let space: String = " ".repeat(text.chars().count() + 2);
    std::process::Command::new("clear").status().unwrap();
    println!("{dash}");
    println!(" {text}{dash}");
    println!("{space}\\    ^__^");
    println!("{space} \\   (oo)\\_______");
    println!("{space}     (__)\\       )\\/\\");
    println!("{space}          ||____w |");
    println!("{space}          ||     ||");
}