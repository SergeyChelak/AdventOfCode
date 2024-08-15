use std::collections::{HashMap, HashSet};

pub const REGISTERS_COUNT: usize = 6;
pub type MachineInt = isize;
pub type Registers = [MachineInt; REGISTERS_COUNT];
pub type Instruction = [MachineInt; 4];

type MachineInstruction = dyn Fn(&mut Machine);

pub struct Machine {
    reg: Registers,
    args: Instruction,
    instruction: Vec<&'static MachineInstruction>,
    mapping: HashMap<MachineInt, usize>,
    last_modified_register: Option<usize>,
    debug: bool,
}

impl Default for Machine {
    fn default() -> Self {
        let mut machine = Self::new();
        machine.set_default_mapping();
        machine
    }
}

impl Machine {
    pub fn new() -> Self {
        let instruction: Vec<&'static MachineInstruction> = vec![
            &Self::addr,
            &Self::addi,
            &Self::mulr,
            &Self::muli,
            &Self::banr,
            &Self::bani,
            &Self::borr,
            &Self::bori,
            &Self::setr,
            &Self::seti,
            &Self::gtir,
            &Self::gtri,
            &Self::gtrr,
            &Self::eqir,
            &Self::eqri,
            &Self::eqrr,
        ];
        Self {
            reg: Registers::default(),
            args: Instruction::default(),
            instruction,
            mapping: HashMap::new(),
            last_modified_register: None,
            debug: false,
        }
    }

    fn set_default_mapping(&mut self) {
        (0..self.instruction.len()).for_each(|i| {
            self.mapping.insert(i as MachineInt, i);
        });
    }

    pub fn _set_debug(&mut self, is_enabled: bool) {
        self.debug = is_enabled;
    }

    fn debug(&self, name: &str) {
        if !self.debug {
            return;
        }
        let transl = match name {
            "seti" => {
                // seti (set immediate) stores value A into register C. (Input B is ignored.)
                format!("R{} = {}", self.idx_c(), self.val_a())
            }
            "addr" => {
                // addr (add register) stores into register C the result of adding register A and register B.
                format!("R{} = R{} + R{}", self.idx_c(), self.idx_a(), self.idx_b())
            }
            "addi" => {
                // addi (add immediate) stores into register C the result of adding register A and value B.
                format!("R{} = R{} + {}", self.idx_c(), self.idx_a(), self.val_b())
            }
            "mulr" => {
                format!("R{} = R{} * R{}", self.idx_c(), self.idx_a(), self.idx_b())
            }
            "gtrr" => {
                // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
                format!(
                    "R{} = R{} > R{} ? 1 : 0",
                    self.idx_c(),
                    self.idx_a(),
                    self.idx_b()
                )
            }
            "eqrr" => {
                format!(
                    "R{} = R{} == R{} ? 1 : 0",
                    self.idx_c(),
                    self.idx_a(),
                    self.idx_b(),
                )
            }
            "muli" => {
                // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
                format!("R{} = R{} * {}", self.idx_c(), self.idx_a(), self.val_b())
            }
            _ => {
                format!("! {} ? ? ?", name)
            }
        };
        print!(
            "{name} {} {} {}\t{transl:25}",
            self.args[1], self.args[2], self.args[3]
        );
        println!("\t{:?}", self.reg);
    }

    fn addr(&mut self) {
        self.debug("addr");
        // addr (add register) stores into register C the result of adding register A and register B.
        self.set_reg(
            self.idx_c(),
            self.reg[self.idx_a()] + self.reg[self.idx_b()],
        );
    }

    fn addi(&mut self) {
        self.debug("addi");
        // addi (add immediate) stores into register C the result of adding register A and value B.
        self.set_reg(self.idx_c(), self.reg[self.idx_a()] + self.val_b());
    }

    fn mulr(&mut self) {
        self.debug("mulr");
        // mulr (multiply register) stores into register C the result of multiplying register A and register B.
        self.set_reg(
            self.idx_c(),
            self.reg[self.idx_a()] * self.reg[self.idx_b()],
        );
    }

    fn muli(&mut self) {
        self.debug("muli");
        // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
        self.set_reg(self.idx_c(), self.reg[self.idx_a()] * self.val_b());
    }

    fn banr(&mut self) {
        self.debug("banr");
        // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
        self.set_reg(
            self.idx_c(),
            self.reg[self.idx_a()] & self.reg[self.idx_b()],
        );
    }

    fn bani(&mut self) {
        self.debug("bani");
        // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
        self.set_reg(self.idx_c(), self.reg[self.idx_a()] & self.val_b());
    }

    fn borr(&mut self) {
        self.debug("borr");
        // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
        self.set_reg(
            self.idx_c(),
            self.reg[self.idx_a()] | self.reg[self.idx_b()],
        );
    }

    fn bori(&mut self) {
        self.debug("bori");
        // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
        self.set_reg(self.idx_c(), self.reg[self.idx_a()] | self.val_b());
    }

    fn setr(&mut self) {
        self.debug("setr");
        // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
        self.set_reg(self.idx_c(), self.reg[self.idx_a()]);
    }

    fn seti(&mut self) {
        self.debug("seti");
        // seti (set immediate) stores value A into register C. (Input B is ignored.)
        self.set_reg(self.idx_c(), self.val_a());
    }

    fn gtir(&mut self) {
        self.debug("gtir");
        // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
        self.set_reg(
            self.idx_c(),
            (self.val_a() > self.reg[self.idx_b()]) as MachineInt,
        );
    }

    fn gtri(&mut self) {
        self.debug("gtri");
        // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
        self.set_reg(
            self.idx_c(),
            (self.reg[self.idx_a()] > self.val_b()) as MachineInt,
        );
    }

    fn gtrr(&mut self) {
        self.debug("gtrr");
        // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
        self.set_reg(
            self.idx_c(),
            (self.reg[self.idx_a()] > self.reg[self.idx_b()]) as MachineInt,
        );
    }

    fn eqir(&mut self) {
        self.debug("eqir");
        // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
        self.set_reg(
            self.idx_c(),
            (self.val_a() == self.reg[self.idx_b()]) as MachineInt,
        );
    }

    fn eqri(&mut self) {
        self.debug("eqri");
        // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
        self.set_reg(
            self.idx_c(),
            (self.reg[self.idx_a()] == self.val_b()) as MachineInt,
        );
    }

    fn eqrr(&mut self) {
        self.debug("eqrr");
        // eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
        self.set_reg(
            self.idx_c(),
            (self.reg[self.idx_a()] == self.reg[self.idx_b()]) as MachineInt,
        );
    }

    // Accessors
    fn idx_a(&self) -> usize {
        self.args[1] as usize
    }

    fn idx_b(&self) -> usize {
        self.args[2] as usize
    }

    fn idx_c(&self) -> usize {
        self.args[3] as usize
    }

    fn val_a(&self) -> MachineInt {
        self.args[1]
    }

    fn val_b(&self) -> MachineInt {
        self.args[2]
    }

    // -------
    pub fn ambiguous_count(&mut self, data: &TraceData) -> usize {
        let mut count = 0usize;
        for f in self.instruction.clone() {
            self.reg = data.before;
            self.args = data.instr;
            f(self);
            if self.reg == data.after {
                count += 1;
            }
        }
        count
    }

    // -------
    pub fn try_remap(&mut self, data: &TraceData) {
        let opcode = data.instr[0];
        if self.mapping.contains_key(&opcode) {
            return;
        }
        let mapped = self.mapping.values().cloned().collect::<HashSet<usize>>();
        let mut count = 0usize;
        let mut idx = 0usize;
        for (i, f) in self.instruction.clone().iter().enumerate() {
            if mapped.contains(&i) {
                continue;
            }
            self.reg = data.before;
            self.args = data.instr;
            f(self);
            if self.reg == data.after {
                count += 1;
                idx = i;
            }
        }
        if count == 1 {
            self.mapping.insert(opcode, idx);
        }
    }

    pub fn reset(&mut self) {
        self.reg.iter_mut().for_each(|x| *x = 0);
    }

    pub fn exec(&mut self, args: Instruction) {
        let opcode = args[0];
        self.args = args;
        let Some(idx) = self.mapping.get(&opcode) else {
            panic!("Instruction {opcode} not mapped!")
        };
        self.instruction[*idx](self)
    }

    pub fn instructions_count(&self) -> usize {
        self.instruction.len()
    }

    pub fn remap_count(&self) -> usize {
        self.mapping.len()
    }

    pub fn reg(&self, index: usize) -> MachineInt {
        self.reg[index]
    }

    pub fn _regs(&self) -> &Registers {
        &self.reg
    }

    pub fn set_reg(&mut self, index: usize, value: MachineInt) {
        self.reg[index] = value;
        self.last_modified_register = Some(index);
    }

    pub fn last_modified_register(&self) -> Option<usize> {
        self.last_modified_register
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct TraceData {
    pub before: Registers,
    pub instr: Instruction,
    pub after: Registers,
}

#[derive(Clone)]
#[repr(C)]
pub enum Operation {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Operation::*;
        match value {
            "addr" => Ok(Addr),
            "addi" => Ok(Addi),
            "mulr" => Ok(Mulr),
            "muli" => Ok(Muli),
            "banr" => Ok(Banr),
            "bani" => Ok(Bani),
            "borr" => Ok(Borr),
            "bori" => Ok(Bori),
            "setr" => Ok(Setr),
            "seti" => Ok(Seti),
            "gtir" => Ok(Gtir),
            "gtri" => Ok(Gtri),
            "gtrr" => Ok(Gtrr),
            "eqir" => Ok(Eqir),
            "eqri" => Ok(Eqri),
            "eqrr" => Ok(Eqrr),
            _ => Err(format!("Unknown operation {}", value)),
        }
    }
}
