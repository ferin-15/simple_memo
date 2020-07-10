extern crate chrono;

use std::env;
use std::process;
use std::io::{Write, ErrorKind};
use std::fs::{File, OpenOptions};
use chrono::{Local, DateTime};

fn main() {
    let args: Vec<String> = env::args().collect();

    // 入力をparse
    let memo = Memo::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem making memo data: {}", err);
        process::exit(1);
    });

    // ファイルに書き込み
    if let Err(e) = add_memo(&memo) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

struct Memo {
    timestamp: DateTime<Local>,
    category: String,
    body: String,
}

impl Memo {
    fn new(args: &[String]) -> Result<Memo, &'static str> {
        if args.len() == 0 {
            return Err("not enough arguments");
        }

        let timestamp = Local::now();
        let body = args[1].clone();
        let category = args[2].clone();

        Ok(Memo { timestamp, category, body })
    }

    fn to_string(&self) -> String {
        format!("timestamp: {}\ncategory: {}\nbody: {}\n", self.timestamp, self.category, self.body)
    }
}

fn add_memo(memo: &Memo) -> Result<(), String> {
    let file = OpenOptions::new()
        .append(true)
        .open("memo.txt");
    let mut file = match file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {  // memo.txtが存在しない場合新規作成
            match File::create("memo.txt") {
                Ok(fc) => fc,
                Err(e) => return Err(format!("Problem making file: {:?}", e)),
            }
        },
        Err(error) => return Err(format!("Problem opening file: {:?}", error)),
    };

    file.write_all(memo.to_string().as_bytes()).unwrap();   // memo.txt に書き込み

    Ok(())
}