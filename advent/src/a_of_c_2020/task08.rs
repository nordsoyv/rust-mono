use crate::task::Task;
use crate::util::read_file;
use crate::a_of_c_2020::task08::Cmd::{Jmp, Nop};

pub struct Task08A {}

pub struct Task08B {}

impl Task for Task08A {
  fn run(&self) {
    let mut game = GameConsole::new();
    game.read_program_from_string(&read_file("./res/2020/task08.txt"));
    game.run();
  }
}

impl Task for Task08B {
  fn run(&self) {
    let mut game = GameConsole::new();
    game.read_program_from_string(&read_file("./res/2020/task08.txt"));
    for com_num in 0..game.commands.len() {
      match &game.commands[com_num] {
        Cmd::Nop(arg) => {
          game.change_nop_to_jmp(com_num);
          let value = game.run();
          match value {
            ReturnCode::EOF(val) => {
              println!("Value on EOF {}", val);
              return;
            }
            _ => {}
          }
          game.change_jmp_to_nop(com_num);
        }
        Cmd::Jmp(arg) => {
          game.change_jmp_to_nop(com_num);
          let value = game.run();
          match value {
            ReturnCode::EOF(val) => {
              println!("Value on EOF {}", val);
              return;
            }
            _ => {}
          }
          game.change_nop_to_jmp(com_num);
        }
        _ => {}
      }
    }
  }
}

#[derive(Debug, PartialEq)]
enum Cmd {
  Nop(i32),
  Acc(i32),
  Jmp(i32),
}

#[derive(Debug, PartialEq)]
enum ReturnCode {
  EOF(i32),
  Duplicate(i32),
}

#[derive(Debug)]
struct GameConsole {
  commands: Vec<Cmd>,
  ip: i32,
  acc: i32,
}

impl GameConsole {
  fn new() -> GameConsole {
    GameConsole {
      commands: vec![],
      ip: 0,
      acc: 0,
    }
  }
  pub fn read_program_from_string(&mut self, input: &str) {
    self.commands = input.lines().map(|line| parse_cmd(line)).collect::<Vec<Cmd>>();
  }

  pub fn run(&mut self) -> ReturnCode {
    self.acc = 0;
    let commands = &self.commands;
    let mut cmd_count = vec![0; commands.len()];
    let mut ip: i32 = 0;
    loop {
      if ip == commands.len() as i32 {
        return ReturnCode::EOF(self.acc);
      }
      let cmd = &commands[ip as usize];
      cmd_count[ip as usize] += 1;
      if cmd_count[ip as usize] > 1 {
        return ReturnCode::Duplicate(self.acc);
      }
      match cmd {
        Cmd::Jmp(arg) => ip += arg,
        Cmd::Acc(arg) => {
          self.acc += arg;
          ip += 1;
        }
        Cmd::Nop(_arg) => ip += 1,
      }
    }
  }

  pub fn change_nop_to_jmp(&mut self, index : usize) {
    let old_cmd = &self.commands[index];
    match old_cmd {
      Cmd::Nop(arg )=>self.commands[index] = Jmp(*arg),
      _ => panic!()
    }

  }
  pub fn change_jmp_to_nop(&mut self, index : usize) {
    let old_cmd = &self.commands[index];
    match old_cmd {
      Cmd::Jmp(arg )=>self.commands[index] = Cmd::Nop(*arg),
      _ => panic!()
    }

  }
}

fn parse_cmd(input: &str) -> Cmd {
  let items = input.split(" ").collect::<Vec<&str>>();
  let arg = items[1].parse::<i32>().unwrap();
  match items[0] {
    "nop" => Cmd::Nop(arg),
    "acc" => Cmd::Acc(arg),
    "jmp" => Cmd::Jmp(arg),
    _ => panic!("Unknown command found {}", items[0])
  }
}

#[cfg(test)]
mod test {
  use crate::a_of_c_2020::task08::{Cmd, GameConsole, parse_cmd, ReturnCode};
  use crate::a_of_c_2020::task08::Cmd::{Acc, Jmp, Nop};

  #[test]
  fn test_parse_cmd() {
    assert_eq!(parse_cmd("nop +0"), Cmd::Nop(0));
    assert_eq!(parse_cmd("nop -1"), Cmd::Nop(-1));
    assert_eq!(parse_cmd("nop +1"), Cmd::Nop(1));
    assert_eq!(parse_cmd("acc +1"), Cmd::Acc(1));
    assert_eq!(parse_cmd("acc +0"), Cmd::Acc(0));
    assert_eq!(parse_cmd("acc -1"), Cmd::Acc(-1));
    assert_eq!(parse_cmd("jmp +1"), Cmd::Jmp(1));
    assert_eq!(parse_cmd("jmp +0"), Cmd::Jmp(0));
    assert_eq!(parse_cmd("jmp -1"), Cmd::Jmp(-1));
  }

  #[test]
  fn test_read_program() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let mut game_console = GameConsole::new();
    game_console.read_program_from_string(input);
    assert_eq!(game_console.commands, vec![Nop(0), Acc(1), Jmp(4), Acc(3), Jmp(-3), Acc(-99), Acc(1), Jmp(-4), Acc(6)]);
  }

  #[test]
  fn test_run_program() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let mut game_console = GameConsole::new();
    game_console.read_program_from_string(input);
    let value = game_console.run();
    assert_eq!(value, ReturnCode::Duplicate(5));
  }

  #[test]
  fn test_run_program2() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
    let mut game_console = GameConsole::new();
    game_console.read_program_from_string(input);
    let value = game_console.run();
    assert_eq!(value, ReturnCode::EOF(8));
  }
}