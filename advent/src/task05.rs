use crate::task::Task;
use crate::int_code::{int_code_reader, IntCodeMachine};

pub struct Task05A {}
pub struct Task05B {}

impl Task for Task05A {
    fn run(&self) {
        let int_code = int_code_reader("./res/task05.txt");
        let mut machine = IntCodeMachine::new();
        machine.set_code(int_code);
        machine.run();
    }
}

impl Task for Task05B {
    fn run(&self) {
        unimplemented!()
    }
}
