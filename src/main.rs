extern crate chrono;
extern crate simple_memo;

use std::env;
use std::process;

use simple_memo::Memo;
use simple_memo::InputType;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = simple_memo::parse(&args);
    match input {
        Err(err) => {
            eprintln!("parse Problem: {}", err);
            process::exit(1);
        },
        Ok(input_type) => {
            match input_type {
                InputType::MemoT(m) => {
                    if let Err(e) = Memo::write(&m) {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                },
                InputType::ListT(num) => {
                    if let Err(e) = simple_memo::show_list(num) {
                        eprintln!("Application error: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }
}

