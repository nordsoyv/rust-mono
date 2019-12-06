use crate::task::Task;
use crate::int_code::{int_code_reader, IntCodeMachine};

pub struct Task05A {}
pub struct Task05B {}

impl Task for Task05A {
    fn run(&self) {
        let mut int_code = int_code_reader("./res/task05.txt");
        let mut machine = IntCodeMachine::new();
//  int_code[1] = 42;
//  int_code[2] = 59;

        machine.set_code(int_code);
        machine.run();
//        assert_eq!(machine.get_memory(0), 19690720);
    }
}

impl Task for Task05B {
    fn run(&self) {
        unimplemented!()
    }
}
