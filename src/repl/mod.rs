use std;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::num::ParseIntError;
use std::path::Path;

use crate::assembler::program_parser::program;
use crate::assembler::Assembler;
use crate::vm::VM;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    asm: Assembler,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
            asm: Assembler::new(),
        }
    }

    pub fn run(&mut self) {
        println!("welcome to this painfully uninteresting vm");
        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("error flushing stdout");

            stdin
                .read_line(&mut buffer)
                .expect("error reading line from user");
            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".quit" => {
                    println!("later skater");
                    std::process::exit(0);
                }
                ".hist" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                ".prog" => {
                    println!("listing instructions in vm right now");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                ".clear_prog" => {
                    println!("Removing all bytes from VM's program vector...");
                    self.vm.program.truncate(0);
                    println!("Done!");
                }
                ".clear_regs" => {
                    println!("Setting all registers to 0");
                    for i in 0..self.vm.registers.len() {
                        self.vm.registers[i] = 0;
                    }
                    println!("Done!");
                }
                ".dump_regs" => {
                    println!("dumping registers and contents");
                    println!("{:#?}", self.vm.registers);
                    println!("End of Register Listing")
                }
                ".symbols" => {
                    println!("now showing symbols table");
                    println!("{:#?}", self.asm.symbols);
                    println!("End of Symbols Listing");
                }
                ".load" => {
                    print!("enter file path: ");
                    io::stdout().flush().expect("error flushing stdout");
                    let mut tmp = String::new();
                    stdin
                        .read_line(&mut tmp)
                        .expect("Unable to read line from user");
                    println!("Attempting to load program from file...");
                    let tmp = tmp.trim();
                    let filename = Path::new(&tmp);
                    let mut f = File::open(Path::new(&filename)).expect("File not found");
                    let mut contents = String::new();
                    f.read_to_string(&mut contents)
                        .expect("There was an error reading from the file");
                    match self.asm.assemble(&contents) {
                        Some(mut assembled_program) => {
                            println!("Sending assembled program to VM");
                            self.vm.program.append(&mut assembled_program);
                            println!("{:#?}", self.vm.program);
                            self.vm.run();
                        }
                        None => {
                            println!("Unable to parse input");
                            continue;
                        }
                    }
                }
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((_remainder, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                            continue;
                        }
                    };

                    self.vm
                        .program
                        .append(&mut program.to_bytes(&self.asm.symbols));
                    self.vm.run_once();
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

