use std::fs;

pub type IntCode = Vec<i32>;

pub struct IntCodeMachine {
  code: IntCode,
}

impl IntCodeMachine {
  pub fn new() -> IntCodeMachine { IntCodeMachine { code: vec![] } }


  pub fn set_code(&mut self, new_code: IntCode) {
    self.code = new_code.clone();
  }

  pub fn run(&mut self) {
    let mut instruction_pointer = 0;
    loop {
      let op = self.code[instruction_pointer];
      if op == 99 {
        return;
      }
      let arg1 = self.code[instruction_pointer + 1] as usize;
      let arg1_value = self.code[arg1];
      let arg2 = self.code[instruction_pointer + 2] as usize;
      let arg2_value = self.code[arg2];
      let loc = self.code[instruction_pointer + 3] as usize;

      match op {
        1 => self.code[loc] = arg1_value + arg2_value,
        2 => self.code[loc] = arg1_value * arg2_value,
        _ => panic!(format!("Unknown op code. pos: {}", instruction_pointer)),
      }

      instruction_pointer += 4;
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