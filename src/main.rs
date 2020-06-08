use std::env;
use std::fs;

struct Program {
    token: i64,
    src: String,
    oldsrc: String,
    poolsize: i64,
    line: i64
}

impl Program {
    fn next(&self) {
        return;
    }

    fn expression(&self, level: i64) {

    }

    fn program(&self) {
        &self.next();
        while self.token > 0 {
            println!("Token is: {}", self.token);
            &self.next();
        }
    }
    
    fn eval(&self) -> i32 {
        return 0;
    }
}

pub fn run() -> i32 {
    let args: Vec<String> = env::args().collect();
    let index: i32;
    let mut file_data: String;
    let program = Program{ token: 0, src: String::from(""), oldsrc: String::from(""), poolsize: 256 * 1024, line: 1};

    file_data = fs::read_to_string(&args[1])
        .expect("Something went wrong reading file.");

    file_data.push('0');
    
    program.program();
    return program.eval();
}

fn main() {
    run();
}
