use crate::task::Task;
use crate::int_code::{int_code_reader, IntCodeMachine};
use std::io;

pub struct Task05A {}
pub struct Task05B {}

impl Task for Task05A {
    fn run(&self) {
        let int_code = int_code_reader("./res/task05.txt");
        let mut machine = IntCodeMachine::new();
        machine.set_code(int_code);
        let stdin  = io::stdin();
        let input = stdin.lock();
        let stdout  = io::stdout();
        let out = stdout.lock();

        machine.run(input, out);
    }
}

impl Task for Task05B {
    fn run(&self) {
        unimplemented!()
    }
}
