mod render;
mod calculate;
use std::io;

fn start_menu() {
    let start_options: [&str; 2] = ["quit", "start"];
    let mut index: i8;
    index = 0;
    for option in start_options {
        println!("{}) {}", index, option);
        index += 1;
    };
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input_check = input.chars().next().unwrap();
    match input_check {
        '0' => println!("closing game..."),
        '1' => println!("starting game..."),
        _   => println!("input error")
    };
}

fn main() {
    println!("TEST RUN NOT RELEASE BUILD");
    start_menu();
}