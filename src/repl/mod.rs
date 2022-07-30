use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use nom::types::CompleteStr;
use crate::assembler::program_parser::{Program, program};

pub struct REPL {
    cmd_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            cmd_buffer: vec![],
        }
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
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

    pub fn run(&mut self) {
        println!("this is grain. is this thing on?");
        loop {
            // TODO: Figure out how allocate this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!("");
            io::stdout().flush().expect("unable to flush stdout");

            stdin.read_line(&mut buffer).expect("unable to read line");
            let buffer = buffer.trim();
            self.cmd_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("later loser");
                    std::process::exit(0);
                }
                ".hist" => {
                    for command in &self.cmd_buffer {
                        println!("{}", command);
                    }
                }
                ".prog" => {
                    println!("printing instructions in program vector");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("<DONE>");
                }
                ".dump" => {
                    println!("dumping registers and contents");
                    println!("{:#?}", self.vm.registers);
                    println!("<DONE>")
                }
                _ => {
                    let parsed_program = program(CompleteStr(buffer));

                    if !parsed_program.is_ok() {
                        println!("Unable to parse input");
                        continue;
                    }
                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes();
                    // TODO: Make a function to let us add multiple bytes to the VM
                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }
}
