#[derive(Debug, Clone)]
pub enum Op {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

pub trait Hook {
    fn check(&self, vm: &Vm) -> bool;
}

pub struct Vm {
    pub program: Vec<Op>,
    pub hit_count: Vec<usize>,
    pub hooks: Vec<Box<dyn Hook>>,
    pub counter: usize,
    pub acc: i64,
}

impl Vm {
    pub fn new(program: Vec<Op>) -> Self {
        let hit_count = vec![0; program.len()];
        Vm {
            program,
            hit_count,
            hooks: Vec::new(),
            counter: 0,
            acc: 0,
        }
    }

    pub fn run(&mut self) -> bool {
        enum Cont {
            Jmp(i64),
            Next,
        }
        while self.counter < self.program.len() {
            self.hit_count[self.counter] += 1;
            let cont = match self.program[self.counter] {
                Op::Nop(_) => Cont::Next,
                Op::Acc(val) => {
                    self.acc += val;
                    Cont::Next
                }
                Op::Jmp(offs) => Cont::Jmp(offs),
            };
            for bp in self.hooks.iter() {
                if bp.check(self) {
                    return false;
                }
            }
            self.counter = match cont {
                Cont::Next => self.counter + 1,
                Cont::Jmp(offs) => {
                    let nc = (self.counter as i64) + offs;
                    if nc < 0 || (nc as usize) > self.program.len() {
                        return false;
                    }
                    nc as usize
                }
            };
        }
        println!("terminated");
        self.counter >= self.program.len()
    }
}

pub mod hooks {
    use super::*;
    pub struct Tracing;
    impl Hook for Tracing {
        fn check(&self, vm: &Vm) -> bool {
            println!(
                "exec: acc: {}: {} {:?}",
                vm.acc, vm.counter, vm.program[vm.counter],
            );
            false
        }
    }
}
