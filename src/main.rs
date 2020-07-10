extern crate chrono;
extern crate simple_memo;

use std::env;
use std::process;

use simple_memo::Memo;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 入力をparse
    let memo = Memo::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem making memo data: {}", err);
        process::exit(1);
    });

    // ファイルに書き込み
    if let Err(e) = Memo::add_memo(&memo) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

