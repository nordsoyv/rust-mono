use std::fs;

pub type IntCode = Vec<i32>;

pub struct IntCodeMachine {
  code: IntCode,
  instruction_pointer: usize,
}

#[derive(Clone, Copy, Debug)]
enum InstructionCode {
  Add = 1,
  Multiply = 2,
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

  fn decode_instruction(&self) -> Instruction {
    let op_num = self.code[self.instruction_pointer];


    let s = op_num.to_string();
    let op_digits: Vec<_> = s.chars().map(|d| d.to_digit(10).unwrap()).collect();
    let op = if op_digits.len() == 1 {
      op_digits[op_digits.len()-1]
    }else {
      op_digits[op_digits.len()-1] +( 10 * op_digits[op_digits.len()-2])
    };

    match op {
      1 => {
        let mut args = vec![];
        args.push(Parameter { mode: ParameterMode::Position, value: self.code[self.instruction_pointer + 1] });
        args.push(Parameter { mode: ParameterMode::Position, value: self.code[self.instruction_pointer + 2] });
        args.push(Parameter { mode: ParameterMode::Immediate, value: self.code[self.instruction_pointer + 3] });
        Instruction {
          code: InstructionCode::Add,
          params: args,
          size: 4,
        }
      }
      2 => {
        let mut args = vec![];
        args.push(Parameter { mode: ParameterMode::Position, value: self.code[self.instruction_pointer + 1] });
        args.push(Parameter { mode: ParameterMode::Position, value: self.code[self.instruction_pointer + 2] });
        args.push(Parameter { mode: ParameterMode::Immediate, value: self.code[self.instruction_pointer + 3] });
        Instruction {
          code: InstructionCode::Multiply,
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
      _ => panic!(format!("Unknown code {}", op) )
    }
  }

  fn get_value_for_parameter(&self, param: Parameter) -> i32 {
    match param.mode {
      ParameterMode::Immediate => param.value,
      ParameterMode::Position => self.code[param.value as usize],
    }
  }

  pub fn run(&mut self) {
    loop {
      let op = self.decode_instruction();
      match op.code {
        InstructionCode::Add => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          let arg2_value = self.get_value_for_parameter(op.params[1]);
          let loc = self.get_value_for_parameter(op.params[2]);

          self.code[loc as usize] = arg1_value + arg2_value
        }
        InstructionCode::Multiply => {
          let arg1_value = self.get_value_for_parameter(op.params[0]);
          let arg2_value = self.get_value_for_parameter(op.params[1]);
          let loc = self.get_value_for_parameter(op.params[2]);
          self.code[loc as usize] = arg1_value * arg2_value
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

  machine.set_code(int_code);
  machine.run();
  assert_eq!(machine.get_memory(0), 5866663);
}

#[test]
fn task02b() {
  let mut int_code = int_code_reader("./res/task02.txt");
  let mut machine = IntCodeMachine::new();
  int_code[1] = 42;
  int_code[2] = 59;

  machine.set_code(int_code);
  machine.run();
  assert_eq!(machine.get_memory(0), 19690720);
}