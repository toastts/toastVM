use crate::instruction::Opcode;

#[derive(Default)]
pub struct VM {
    pub registers: [i32; 32],
    pc: usize,
    pub program: Vec<u8>,
    heap: Vec<u8>,
    remainder: usize,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            heap: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    pub fn add_bytes(&mut self, mut b: Vec<u8>) {
        self.program.append(&mut b);
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = u32::from(self.next_16_bits());
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.remainder = (register1 % register2) as usize;
            }
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::IGL => {
                println!("Illegal instruction encountered");
                return true;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= value;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 != register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 > register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 >= register2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false
                }
                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 < register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 <= register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::JMPE => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                } else {
                }
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::INC => {
                let register_number = self.next_8_bits() as usize;
                self.registers[register_number] += 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DEC => {
                let register_number = self.next_8_bits() as usize;
                self.registers[register_number] -= 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DJMPE => {
                let destination = self.next_16_bits();
                if self.equal_flag {
                    self.pc = destination as usize;
                } else {
                    self.next_8_bits();
                }
            }
        };
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result =
            ((u16::from(self.program[self.pc])) << 8) | u16::from(self.program[self.pc + 1]);
        self.pc += 2;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![254, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![2, 1, 0, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![4, 1, 0, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 4;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 5, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_jmpb_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1, 0, 0];
        test_vm.run_once();
        test_vm.run_once();
        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_neq_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 20;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gte_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![11, 0, 1, 0, 11, 0, 1, 0, 11, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_gt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_lte_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_lt_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 20;
        test_vm.registers[1] = 10;
        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[0] = 10;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
        test_vm.registers[0] = 5;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jmpe_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![15, 0, 0, 0, 15, 0, 0, 0, 15, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_aloc_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 1024;
        test_vm.program = vec![17, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 1024);
    }
}
