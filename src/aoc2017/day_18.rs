use crate::solution::Solution;
use crate::utils::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

use super::vm_utils::*;

trait Module {
    fn op_snd(&mut self, value: Value);

    fn op_rcv(&mut self, reg_value: &mut Value) -> bool;

    fn output(&self) -> Option<Value>;
}

#[derive(Default)]
struct SoundModule {
    sound_freq: Option<Value>,
}

impl Module for SoundModule {
    fn op_snd(&mut self, value: Value) {
        self.sound_freq = Some(value);
    }

    fn op_rcv(&mut self, reg_value: &mut Value) -> bool {
        *reg_value == 0
    }

    fn output(&self) -> Option<Value> {
        self.sound_freq
    }
}

struct Message {
    sender: Value,
    value: Value,
}

struct MessageQueue {
    messages: Vec<Message>,
    last_access_idx: HashMap<Value, usize>, // machine id, last message index
}

impl MessageQueue {
    fn new() -> Self {
        Self {
            messages: Vec::new(),
            last_access_idx: HashMap::new(),
        }
    }

    fn send(&mut self, message: Message) {
        self.messages.push(message)
    }

    fn get_message(&mut self, receiver_id: Value) -> Option<Value> {
        let last = self.last_access_idx.get(&receiver_id).unwrap_or(&0);
        let data = self
            .messages
            .iter()
            .enumerate()
            .find(|(idx, msg)| msg.sender != receiver_id && idx >= last);
        if let Some((idx, message)) = data {
            self.last_access_idx.insert(receiver_id, idx + 1);
            Some(message.value)
        } else {
            None
        }
    }
}

struct MessageQueueModule {
    sender_id: Value,
    send_count: Value,
    queue: Rc<RefCell<MessageQueue>>,
}

impl Module for MessageQueueModule {
    fn op_snd(&mut self, value: Value) {
        self.send_count += 1;
        let msg = Message {
            sender: self.sender_id,
            value,
        };
        let mut queue = self.queue.borrow_mut();
        queue.send(msg);
    }

    fn op_rcv(&mut self, reg_value: &mut Value) -> bool {
        let mut queue = self.queue.borrow_mut();
        if let Some(value) = queue.get_message(self.sender_id) {
            *reg_value = value;
            true
        } else {
            false
        }
    }

    fn output(&self) -> Option<Value> {
        Some(self.send_count)
    }
}

impl MessageQueueModule {
    fn new(sender_id: Value, queue: Rc<RefCell<MessageQueue>>) -> Self {
        Self {
            sender_id,
            send_count: 0,
            queue,
        }
    }
}

struct Machine<'a> {
    register: [Value; 26],
    pc: usize,
    loop_count: usize,
    ops: &'a [Op],
    is_suspended: bool,
    module: Box<dyn Module>,
}

impl<'a> Machine<'a> {
    fn new(ops: &'a [Op], module: Box<dyn Module>) -> Self {
        Self {
            register: [0; 26],
            pc: 0,
            loop_count: 0,
            ops,
            is_suspended: false,
            module,
        }
    }

    fn set_program_id(&mut self, id: Value) {
        self.register[(b'p' - b'a') as usize] = id;
    }

    fn run(&mut self) {
        while self.pc < self.ops.len() && !self.is_suspended {
            self.loop_count += 1;
            match &self.ops[self.pc] {
                Op::Snd(op_value) => self.op_snd(op_value),
                Op::Set(reg, op_value) => self.op_set(*reg, op_value),
                Op::Add(reg, op_value) => self.op_add(*reg, op_value),
                Op::Mul(reg, op_value) => self.op_mul(*reg, op_value),
                Op::Mod(reg, op_value) => self.op_mod(*reg, op_value),
                Op::Rcv(reg) => self.op_rcv(*reg),
                Op::Jgz(op_value, offset) => self.op_jgz(op_value, offset),
            }
        }
    }

    fn op_set(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] = val;
        self.pc += 1;
    }

    fn op_add(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] += val;
        self.pc += 1;
    }

    fn op_mul(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] *= val;
        self.pc += 1;
    }

    fn op_mod(&mut self, reg: usize, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.register[reg] %= val;
        self.pc += 1;
    }

    fn op_jgz(&mut self, op_value: &OpValue, offset: &OpValue) {
        let x = self.get_value(op_value);
        if x > 0 {
            let offset = self.get_value(offset);
            self.pc_offset(offset);
        } else {
            self.pc += 1;
        }
    }

    fn op_snd(&mut self, op_value: &OpValue) {
        let val = self.get_value(op_value);
        self.module.op_snd(val);
        self.pc += 1;
    }

    fn op_rcv(&mut self, reg: usize) {
        let val = &mut self.register[reg];
        if !self.module.op_rcv(val) {
            self.is_suspended = true;
        } else {
            self.pc += 1;
        }
    }

    fn get_value(&self, op_value: &OpValue) -> Value {
        match op_value {
            OpValue::Reg(reg) => self.register[*reg],
            OpValue::Val(val) => *val,
        }
    }

    fn pc_offset(&mut self, offset: Value) {
        if offset > 0 {
            self.pc += offset as usize;
        } else {
            let offset = (-offset) as usize;
            self.pc -= offset;
        }
    }

    fn module_output(&self) -> Option<Value> {
        self.module.output()
    }
}

pub struct AoC2017_18 {
    ops: Vec<Op>,
}

impl AoC2017_18 {
    pub fn new() -> io::Result<Self> {
        let ops = read_file_as_lines("input/aoc2017_18")?
            .iter()
            .map(|s| Op::from_str(s))
            .collect();
        Ok(Self { ops })
    }
}

impl Solution for AoC2017_18 {
    fn part_one(&self) -> String {
        let mut machine = Machine::new(&self.ops, Box::<SoundModule>::default());
        machine.run();
        machine.module_output().unwrap().to_string()
    }

    fn part_two(&self) -> String {
        let message_queue = Rc::new(RefCell::new(MessageQueue::new()));
        let create_vm = |program_id: Value| {
            let module = MessageQueueModule::new(program_id, message_queue.clone());
            let mut machine = Machine::new(&self.ops, Box::new(module));
            machine.set_program_id(program_id);
            machine
        };
        let vm_0 = create_vm(0);
        let vm_1 = create_vm(1);
        let vms = [RefCell::new(vm_0), RefCell::new(vm_1)];
        loop {
            let mut is_deadlock = true;
            for vm in &vms {
                let mut vm = vm.borrow_mut();
                let before = vm.loop_count;
                vm.run();
                let after = vm.loop_count;
                if after - before > 1 {
                    is_deadlock = false;
                }
                vm.is_suspended = false;
            }
            if is_deadlock {
                break;
            }
        }
        let result = vms[1].borrow().module_output();
        result.unwrap().to_string()
    }

    fn description(&self) -> String {
        "AoC 2017/Day 18: Duet".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aoc2017_18_input_load_test() -> io::Result<()> {
        let sol = AoC2017_18::new()?;
        assert!(!sol.ops.is_empty());
        Ok(())
    }

    #[test]
    fn aoc2017_18_example1() {
        let ops = str2ops(
            "
            set a 1
            add a 2
            mul a a
            mod a 5
            snd a
            set a 0
            rcv a
            jgz a -1
            set a 1
            jgz a -2",
        );
        assert_eq!(ops.len(), 10);
        let sol = AoC2017_18 { ops };
        assert_eq!(sol.part_one(), "4")
    }

    #[test]
    fn aoc2017_18_example2() {
        let ops = str2ops(
            "
            snd 1
            snd 2
            snd p
            rcv a
            rcv b
            rcv c
            rcv d",
        );
        let sol = AoC2017_18 { ops };
        assert_eq!(sol.part_two(), "3")
    }

    fn str2ops(s: &str) -> Vec<Op> {
        s.split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(Op::from_str)
            .collect()
    }

    #[test]
    fn aoc2017_18_correctness() -> io::Result<()> {
        let sol = AoC2017_18::new()?;
        assert_eq!(sol.part_one(), "9423");
        assert_eq!(sol.part_two(), "7620");
        Ok(())
    }
}
