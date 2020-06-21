use std::env;
use std::fs;

struct Program {
    token: i64,
    src: String,
    oldsrc: String,
    poolsize: i64,
    line: i64,
    pc: Vec<i64>,
    bp: Vec<i64>,
    sp: Vec<i64>,
    ax: Vec<i64>
}

enum Number {
    LEA,
    IMM,
    JMP,
    CALL,
    JZ,
    JNZ,
    ENT,
    ADJ,
    LEV,
    LI,
    LC,
    SI,
    SC,
    PUSH,
    OR,
    XOR,
    AND,
    EQ,
    NE,
    LT,
    GT,
    LE,
    GE,
    SHL,
    SHR,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    OPEN,
    READ,
    CLOS,
    PRTF,
    MALC,
    MSET,
    MCMP,
    EXIT
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
    
    fn eval(&mut self) -> i32 {
        let mut op: i64;
        let tmp: i64;

        while true {
            self.pc += 1;
            op = self.pc as i64;

            if Number::IMM as i64 == op {
                self.pc += 1;
                self.ax = self.pc;
            } else if Number::LC as i64 == op {
                
            }
        }
        return 0;
    }
}

pub fn run() -> i32 {
    let args: Vec<String> = env::args().collect();
    let index: i32;
    let mut file_data: String;
    let program = Program{ token: 0, src: String::from(""), oldsrc: String::from(""), poolsize: 256 * 1024, line: 1, pc: 0, bp: 0, sp: 0, ax: 0};

    file_data = fs::read_to_string(&args[1])
        .expect("Something went wrong reading file.");

    file_data.push('0');
    
    program.program();
    return program.eval();
}

fn main() {
    run();
}
