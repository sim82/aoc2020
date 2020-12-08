use aoc2020::asm_grammar;
use aoc2020::vm::{hooks, Hook, Op, Vm};
use std::io::Read;

struct ExecutedTwice;
impl Hook for ExecutedTwice {
    fn check(&self, vm: &Vm) -> bool {
        if vm.hit_count[vm.counter] > 1 {
            println!("hit twice: {}", vm.counter);
            return true;
        }
        false
    }
}

fn main() {
    let mut code = String::new();
    std::io::stdin().lock().read_to_string(&mut code).unwrap();
    let program: Vec<Op> = asm_grammar::ProgramParser::new().parse(&code[..]).unwrap();

    println!("{:?}", program);

    let mut vm = Vm::new(program.clone());
    let mut hooks: Vec<Box<dyn Hook>> =
        vec![Box::new(ExecutedTwice {}), Box::new(hooks::Tracing {})];
    vm.run(Some(&mut hooks));

    for i in 0..program.len() {
        let mut program = program.clone();

        // rewrite program
        program[i] = match &program[i] {
            Op::Nop(v) => Op::Jmp(*v),
            Op::Jmp(v) => Op::Nop(*v),
            x => x.clone(),
        };
        let mut vm = Vm::new(program);
        let mut hooks: Vec<Box<dyn Hook>> = vec![Box::new(ExecutedTwice {})];
        if vm.run(Some(&mut hooks)) {
            println!("success: {} {}", i, vm.acc);
        }
    }
}
