use std::io::{Write, ErrorKind, BufRead, BufReader};
use std::fs::{File, OpenOptions};
use chrono::{Local, DateTime};
use std::error::Error;

pub enum InputType {
    MemoT(Memo),
    ListT(i32),  // 表示件数を保持 
    SearchT(String), // 検索する単語を保持
}

pub fn parse(args: &[String]) -> Result<InputType, &'static str> {
    if args.len() == 1 {
        return Err("not enough arguments");
    }

    if args[1] == "--list" {
        if args.len() == 2 {
            Ok(InputType::ListT(5))
        } else {
            // 整数以外が入力されたときのエラー処理
            if args[2].trim().parse::<i32>().is_err() {
                return Err("type a number");
            }
            let list_num : i32 = args[2].trim().parse().unwrap();
            Ok(InputType::ListT(list_num))
        }
    } else if args[1] == "--search" {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(InputType::SearchT(args[2].clone()))
    } else {
        // Memo::new() のエラー処理
        let memo = Memo::new(args);
        return match memo {
            Err(e) => Err(e),
            Ok(m) => Ok(InputType::MemoT(m))
        };
    }
}

pub fn search(word: String) -> Result<(), Box<Error>> {
    let mut cnt = 0;
    let mut flag = false;
    let mut pre3 = [String::from(""), String::from(""), String::from("")];
    let mut results : Vec<String> = Vec::new();
    for result in BufReader::new(File::open("memo.txt")?).lines() {
        let line = result?;
        pre3[cnt%3] = line.clone();
        if line.contains(&word) {
            flag = true;
        }
        if cnt%3==2 && flag {
            for i in pre3.iter() {
                results.push(i.to_string());
            }
            flag = false
        }
        cnt += 1;
    }

    show_list(results);

    Ok(())
}

pub fn latest_memo_list(num: i32) -> Result<(), Box<Error>> {
    let mut lines = Vec::new();
    for result in BufReader::new(File::open("memo.txt")?).lines() {
        let line = result?;
        lines.push(line);
    }
    lines.reverse();

    let mut results = Vec::new();
    let mut cnt = 0;
    for line in lines {
        if cnt < 3*num {
            results.push(line);
        }
        cnt += 1;
    }
    results.reverse();

    show_list(results);

    Ok(())
}

pub fn show_list(lists: Vec<String>) {  
    let mut cnt = 0;
    for line in lists {
        println!("{}", line);
        cnt += 1;
        if cnt%3 == 0 {
            println!("");
        }
    }
}

pub struct Memo {
    pub timestamp: DateTime<Local>,
    pub category: Option<String>,
    pub body: String,
}

impl Memo {
    pub fn new(args: &[String]) -> Result<Memo, &'static str> {
        if args.len() == 1 {
            return Err("not enough arguments");
        }

        let timestamp = Local::now();
        let body = args[1].clone();
        let category = if args.len() >= 3 {
            Some(args[2].clone())
        } else {
            None
        };

        Ok(Memo { timestamp, category, body })
    }

    pub fn to_string(&self) -> String {
        if let Some(s) = &self.category {
            format!("timestamp: {}\ncategory: {:?}\nbody: {}\n", self.timestamp, s, self.body)
        } else {
            format!("timestamp: {}\nbody: {}\n", self.timestamp, self.body)
        }
    }

    pub fn write(&self) -> Result<(), String> {
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
    
        file.write_all(self.to_string().as_bytes()).unwrap();   // memo.txt に書き込み
    
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn memo_new_argc1() {
        let args = vec![String::from("filename")];
        let result = Memo::new(&args);

        assert!(result.is_err());
    }

    #[test]
    fn memo_new_argc2() {
        let args = vec![String::from("filename"), String::from("body")];
        let result = Memo::new(&args);

        assert!(result.is_ok());
        if let Ok(memo) = result {
            assert_eq!(memo.body, String::from("body"));
            assert!(memo.category.is_none());
        }
    }

    #[test]
    fn memo_new_argc3() {
        let args = vec![String::from("filename"), String::from("body"), String::from("category")];
        let result = Memo::new(&args);

        assert!(result.is_ok());
        if let Ok(memo) = result {
            assert_eq!(memo.body, String::from("body"));
            assert_eq!(memo.category, Some(String::from("category")));
        }
    }
}