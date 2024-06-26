use std::fs;

pub type IntCode = Vec<i32>;

pub struct IntCodeMachine {
  code: IntCode,
  instruction_pointer: usize,
  pub output: Vec<i32>,
}

#[derive(Clone, Copy, Debug)]
pub enum MachineReturn {
  Halt,
  BreakForInput,
}

#[derive(Clone, Copy, Debug)]
enum InstructionCode {
  Add = 1,
  Multiply = 2,
  Read = 3,
  Print = 4,
  JmpIfTrue = 5,
  JmpIfFalse = 6,
  LessThan = 7,
  Equals = 8,
  Halt = 99,
}

#[derive(Clone, Copy, Debug)]
enum ParameterMode {
  Position,
  Immediate,
}

#[derive(Clone, Copy, Debug)]
struct Parameter {
  mode: ParameterMode,
  value: i32,
}

#[derive(Clone, Debug)]
struct Instruction {
  code: InstructionCode,
  params: Vec<Parameter>,
  size: i32,
}

impl IntCodeMachine {
  pub fn new(code : IntCode) -> IntCodeMachine { IntCodeMachine { code, instruction_pointer: 0, output : vec![] } }

  #[allow(unused)]
  pub fn set_code(&mut self, new_code: IntCode) {
    self.code = new_code.clone();
  }

  fn decode_param(&self, digits: &Vec<u32>, param_num: usize) -> Parameter {
    let mode = if digits.len() > param_num + 1 {
      match digits[digits.len() - 2 - param_num] {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        _ => panic!("Unknown parameter mode {:?} , param {}", digits, param_num)
      }
    } else {
      ParameterMode::Position
    };

    Parameter { mode, value: self.code[self.instruction_pointer + param_num] }
  }

  fn decode_instruction(&self) -> Instruction {
    let op_num = self.code[self.instruction_pointer];
    let s = op_num.to_string();
    let op_digits: Vec<_> = s.chars().map(|d| d.to_digit(10).unwrap()).collect();

    let op = if op_digits.len() == 1 {
      op_digits[op_digits.len() - 1]
    } else {
      op_digits[op_digits.len() - 1] + (10 * op_digits[op_digits.len() - 2])
    };

    match op {
      1 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        args.push(self.decode_param(&op_digits, 2));
        args.push(self.decode_param(&op_digits, 3));
        Instruction {
          code: InstructionCode::Add,
          params: args,
          size: 4,
        }
      }
      2 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        args.push(self.decode_param(&op_digits, 2));
        args.push(self.decode_param(&op_digits, 3));
        Instruction {
          code: InstructionCode::Multiply,
          params: args,
          size: 4,
        }
      }
      3 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        Instruction {
          code: InstructionCode::Read,
          params: args,
          size: 2,
        }
      }
      4 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        Instruction {
          code: InstructionCode::Print,
          params: args,
          size: 2,
        }
      }
      5 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        args.push(self.decode_param(&op_digits, 2));
        Instruction {
          code: InstructionCode::JmpIfTrue,
          params: args,
          size: 3,
        }
      }
      6 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        args.push(self.decode_param(&op_digits, 2));
        Instruction {
          code: InstructionCode::JmpIfFalse,
          params: args,
          size: 3,
        }
      }
      7 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        args.push(self.decode_param(&op_digits, 2));
        args.push(self.decode_param(&op_digits, 3));
        Instruction {
          code: InstructionCode::LessThan,
          params: args,
          size: 4,
        }
      }
      8 => {
        let mut args = vec![];
        args.push(self.decode_param(&op_digits, 1));
        args.push(self.decode_param(&op_digits, 2));
        args.push(self.decode_param(&op_digits, 3));
        Instruction {
          code: InstructionCode::Equals,
          params: args,
          size: 4,
        }
      }
      99 => {
        let args = vec![];
        Instruction {
          code: InstructionCode::Halt,
          params: args,
          size: 1,
        }
      }
      _ => panic!("Unknown code {}", op)
    }
  }

  fn get_value_for_parameter(&self, param: Parameter) -> i32 {
    match param.mode {
      ParameterMode::Immediate => param.value,
      ParameterMode::Position => self.code[param.value as usize],
    }
  }

  fn set_value_for_parameter(&mut self, param: Parameter, value: i32) {
    match param.mode {
      ParameterMode::Immediate => panic!("Trying to write in Immediate mode"),
      ParameterMode::Position => self.code[param.value as usize] = value,
    };
  }

  pub fn run(&mut self, input: &mut Vec<i32>) -> MachineReturn
  {
    loop {
      let op = self.decode_instruction();
      match op.code {
        InstructionCode::Add => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          let arg2_value = self.get_value_for_parameter(op.params[1]);
          let result = arg1_value + arg2_value;
          self.set_value_for_parameter(op.params[2], result)
        }
        InstructionCode::Multiply => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          let arg2_value = self.get_value_for_parameter(op.params[1]);
          let result = arg1_value * arg2_value;
          self.set_value_for_parameter(op.params[2], result)
        }
        InstructionCode::Print => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          self.output.push(arg1_value);
        }
        InstructionCode::Read => {
          if input.len() == 0 {
            return MachineReturn::BreakForInput;
          }
          let value = input.remove(0);
          self.set_value_for_parameter(op.params[0], value);
        }
        InstructionCode::JmpIfTrue => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          if arg1_value != 0 {
            let jmp_target = self.get_value_for_parameter(op.params[1]);
            self.instruction_pointer = jmp_target as usize;
            continue;
          }
        }
        InstructionCode::JmpIfFalse => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          if arg1_value == 0 {
            let jmp_target = self.get_value_for_parameter(op.params[1]);
            self.instruction_pointer = jmp_target as usize;
            continue;
          }
        }
        InstructionCode::LessThan => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          let arg2_value = self.get_value_for_parameter(op.params[1]);
          if arg1_value < arg2_value {
            self.set_value_for_parameter(op.params[2], 1);
          } else {
            self.set_value_for_parameter(op.params[2], 0);
          }
        }
        InstructionCode::Equals => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          let arg2_value = self.get_value_for_parameter(op.params[1]);
          if arg1_value == arg2_value {
            self.set_value_for_parameter(op.params[2], 1);
          } else {
            self.set_value_for_parameter(op.params[2], 0);
          }
        }
        InstructionCode::Halt => return MachineReturn::Halt,
//        _ => panic!(format!("Unknown op code. pos: {}", self.instruction_pointer)),
      }

      self.instruction_pointer += op.size as usize;
    }
  }

  pub fn get_memory(&self, index: usize) -> i32 {
    self.code[index]
  }
}

pub fn int_code_reader(filename: &str) -> IntCode {
  let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
  contents
    .split(",")
    .collect::<Vec<&str>>()
    .iter()
    .map(|n| n.parse::<i32>().unwrap())
    .collect::<Vec<i32>>()
}

fn _print_int_code(code: &IntCode) {
  for chunk in code.chunks(4) {
    println!("{:02?}", chunk);
  }
}

#[test]
fn task02a() {
  let mut int_code = int_code_reader("./res/2019/task02.txt");
  int_code[1] = 12;
  int_code[2] = 2;
  let mut machine = IntCodeMachine::new(int_code);

  machine.run(&mut vec![]);
  assert_eq!(machine.get_memory(0), 5866663);
}

#[test]
fn task02b() {
  let mut int_code = int_code_reader("./res/2019/task02.txt");
  int_code[1] = 42;
  int_code[2] = 59;
  let mut machine = IntCodeMachine::new(int_code);

  machine.run(&mut vec![]);
  assert_eq!(machine.get_memory(0), 19690720);
}


#[test]
fn task05a() {
  let int_code = int_code_reader("./res/2019/task05.txt");
  let mut machine = IntCodeMachine::new(int_code);
  let _output = machine.run( &mut vec![1]);
  println!("{}", machine.output[0]);
  assert_eq!(16225258, machine.output[9])
}


#[test]
fn task05b() {
  let int_code = int_code_reader("./res/2019/task05.txt");
  let mut machine = IntCodeMachine::new(int_code);
  machine.run(&mut vec![5]);
  println!("{}",machine. output[0]);
  assert_eq!(2808771, machine.output[0]);
}

