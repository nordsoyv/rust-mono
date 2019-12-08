use std::{fs, io};
use std::io::{BufRead, Write};

pub type IntCode = Vec<i32>;

pub struct IntCodeMachine {
  code: IntCode,
  instruction_pointer: usize,
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
  pub fn new() -> IntCodeMachine { IntCodeMachine { code: vec![], instruction_pointer: 0 } }

  pub fn set_code(&mut self, new_code: IntCode) {
    self.code = new_code.clone();
  }

  fn decode_param(&self, digits: &Vec<u32>, param_num: usize) -> Parameter {
    let mode = if digits.len() > param_num + 1 {
      match digits[digits.len() - 2 - param_num] {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        _ => panic!(format!("Unknown parameter mode {:?} , param {}", digits, param_num))
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
      _ => panic!(format!("Unknown code {}", op))
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

  pub fn run<R, W>(&mut self, mut input: R, mut out: W)
    where
      R: BufRead,
      W: Write {
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
          writeln!(out, "{}", arg1_value).unwrap();
          //println!(">>>{}", arg1_value);
        }
        InstructionCode::Read => {
          let mut buffer = String::new();
//          write!(out, "Give input (end with EOL) : ").unwrap();
//          out.flush().unwrap();
          // io::stdout().flush().unwrap();
          input.read_line(&mut buffer).unwrap();
          //io::stdin().read_line(&mut buffer).unwrap();
          let value = buffer[..1].parse::<i32>().unwrap();
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
        InstructionCode::Halt => return,
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
  let mut int_code = int_code_reader("./res/task02.txt");
  let mut machine = IntCodeMachine::new();
  int_code[1] = 12;
  int_code[2] = 2;

  let stdin = io::stdin();
  let input = stdin.lock();
  let stdout = io::stdout();
  let out = stdout.lock();
  machine.set_code(int_code);
  machine.run(input, out);
  assert_eq!(machine.get_memory(0), 5866663);
}

#[test]
fn task02b() {
  let mut int_code = int_code_reader("./res/task02.txt");
  let mut machine = IntCodeMachine::new();
  int_code[1] = 42;
  int_code[2] = 59;

  let stdin = io::stdin();
  let input = stdin.lock();
  let stdout = io::stdout();
  let out = stdout.lock();

  machine.set_code(int_code);
  machine.run(input, out);
  assert_eq!(machine.get_memory(0), 19690720);
}


#[test]
fn task05a() {
  let int_code = int_code_reader("./res/task05.txt");
  let mut machine = IntCodeMachine::new();
  let input = b"1\n";
  let mut output = Vec::new();

  machine.set_code(int_code);
  machine.run(&input[..], &mut output);
  let a = String::from_utf8(output).unwrap();
  println!("{}", a);
  assert_eq!("Give input (end with EOL) : >>>0
>>>0
>>>0
>>>0
>>>0
>>>0
>>>0
>>>0
>>>0
>>>16225258
", a);
}


#[test]
fn task05b() {
  let int_code = int_code_reader("./res/task05.txt");
  let mut machine = IntCodeMachine::new();
  let input = b"5\n";
  let mut output = Vec::new();

  machine.set_code(int_code);
  machine.run(&input[..], &mut output);
  let a = String::from_utf8(output).unwrap();
  println!("{}", a);
  assert_eq!("Give input (end with EOL) : >>>2808771\n", a);
}

