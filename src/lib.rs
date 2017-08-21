extern crate hostname;
use std::io::{self, BufRead};
use hostname::get_hostname;

fn input() -> std::string::String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line);

    return line;
}

pub fn sensor(name: std::string::String, f: &Fn() -> std::string::String) {
    loop {
        let line = input();

        if line == "quit\n" {
            break;
        }

        println!("{}:{}:{}", get_hostname().unwrap(), name, f());
        println!("end");
    }
}