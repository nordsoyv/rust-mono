use crate::task::Task;
use crate::int_code::{int_code_reader, IntCodeMachine};

pub struct Task02A {}

pub struct Task02B {}



impl Task for Task02A {
    fn run(&self) {
        let mut int_code = int_code_reader("./res/task02.txt");
        let mut machine = IntCodeMachine::new();
        int_code[1] = 12;
        int_code[2] = 2;

        machine.set_code(int_code);
        machine.run();
        println!("Answer in slot 0 is: {}", machine.get_memory(0));
    }
}

impl Task for Task02B {
    fn run(&self) {
        let org_code = int_code_reader("./res/task02.txt");

        for noun in 0..99 {
            for verb in 0..99 {
                let mut machine = IntCodeMachine::new();
                let mut int_code = org_code.clone();
                int_code[1] = noun;
                int_code[2] = verb;
                machine.set_code(int_code);
                machine.run();
                let answer = machine.get_memory(0);
                if answer == 19690720 {
                    println!("Noun {} Verb {} ", noun, verb);
                    println!("Final answer: {}", (100 * noun) + verb);
                    return;
                }
            }
        }
    }
}
