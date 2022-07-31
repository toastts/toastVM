use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

use crate::assembler::program_parser::program;
use crate::vm::VM;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("welcome to this painfully uninteresting vm");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("can't flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("can't read line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("bye");
                    std::process::exit(0);
                }
                ".hist" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".prog" => {
                    println!("showing instructions in vm");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("<DONE>");
                }
                ".dump" => {
                    println!("dumping registers and content");
                    println!("{:#?}", self.vm.registers);
                    println!("<DONE>")
                }
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((remainder, program)) => {
                            println!("remainder: {:?}", remainder);
                            println!("program is: {:?}", program);
                            program
                        }
                        Err(_) => {
                            println!("can't parse input");
                            continue;
                        }
                    };
                }
            }
        }
    }

    #[allow(dead_code)]
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(' ').collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}

impl Default for REPL {
    fn default() -> Self {
        Self::new()
    }
}

