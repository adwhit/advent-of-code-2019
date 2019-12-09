use std::convert::TryFrom;
use std::io::{self, BufRead, Write};

pub struct Machine {
    data: Vec<i64>,
    ip: usize,
    relative_base: i64,
    input_callback: Box<dyn FnMut() -> i64 + Send>,
    output_callback: Box<dyn FnMut(i64) + Send>,
    initial_size: usize
}

fn read_from_stdin() -> i64 {
    loop {
        print!("Input > ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let line = stdin
            .lock()
            .lines()
            .next()
            .expect("No line")
            .expect("Bad read");
        match line.trim().parse::<i64>() {
            Ok(num) => break num,
            Err(_) => println!("Bad input"),
        }
    }
}

impl Machine {
    pub fn new(data: Vec<i64>) -> Self {
        Self::new_with_io(data, Box::new(read_from_stdin), Box::new(|v| println!("{}", v)))
    }

    pub fn init(mut data: Vec<i64>, v1: i64, v2: i64) -> Self {
        data[1] = v1;
        data[2] = v2;
        Self::new(data)
    }

    pub fn from_file(path: &str) -> Self {
        let data = std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .split(',')
            .map(|line| line.parse::<i64>().unwrap())
            .collect();
        Self::new(data)
    }

    pub fn init_from_file(path: &str, v1: i64, v2: i64) -> Self {
        let machine = Self::from_file(path);
        Machine::init(machine.data, v1, v2)
    }

    pub fn new_with_io(
        mut data: Vec<i64>,
        input: impl FnMut() -> i64 + 'static + Send,
        output: impl FnMut(i64) + 'static + Send,
    ) -> Self {
        let initial_size = data.len();
        data.extend(std::iter::repeat(0).take(1024 * 8));
        Self {
            data,
            ip: 0,
            relative_base: 0,
            input_callback: Box::new(input),
            output_callback: Box::new(output),
            initial_size
        }
    }

    pub fn state(&self) -> &[i64] {
        &self.data[..self.initial_size]
    }

    pub fn step(&mut self) -> Option<i64> {
        match parse_instr(self.fetch(Mode::Immediate)).unwrap() {
            Instr::NoneArg(OpNone::Exit) => return Some(self.get(0)),
            Instr::OneArg(arg, mode1) => match arg {
                OpOne::Input => {
                    let num = (self.input_callback)();
                    self.fetch_and_set(mode1, num);
                }
                OpOne::Output => {
                    let out = self.fetch(mode1);
                    (self.output_callback)(out)
                }
                OpOne::AdjustRelativeBase => {
                    let adj = self.fetch(mode1);
                    self.relative_base += adj;
                }
            },
            Instr::TwoArg(arg, (mode1, mode2)) => {
                let v1 = self.fetch(mode1);
                let v2 = self.fetch(mode2);
                match arg {
                    OpTwo::JumpIfTrue => {
                        if v1 != 0 {
                            self.ip = v2 as usize;
                        }
                    }
                    OpTwo::JumpIfFalse => {
                        if v1 == 0 {
                            self.ip = v2 as usize;
                        }
                    }
                }
            }
            Instr::ThreeArg(arg, (mode1, mode2, mode3)) => {
                let v1 = self.fetch(mode1);
                let v2 = self.fetch(mode2);
                match arg {
                    OpThree::Add => {
                        self.fetch_and_set(mode3, v1 + v2);
                    }
                    OpThree::Mul => {
                        self.fetch_and_set(mode3, v1 * v2);
                    }
                    OpThree::LessThan => self.fetch_and_set(mode3, if v1 < v2 { 1 } else { 0 }),
                    OpThree::Equals => self.fetch_and_set(mode3, if v1 == v2 { 1 } else { 0 }),
                }
            }
        };
        None
    }

    fn fetch(&mut self, mode: Mode) -> i64 {
        let param = self.data[self.ip];
        self.ip += 1;
        match mode {
            Mode::Immediate => param,
            Mode::Position => self.data[param as usize],
            Mode::Relative => self.data[(param + self.relative_base) as usize],
        }
    }

    fn fetch_and_set(&mut self, mode: Mode, val: i64) {
        let ptr = self.data[self.ip];
        match mode {
            Mode::Immediate => unreachable!(),
            Mode::Position => {
                self.set(ptr as usize, val);
            }
            Mode::Relative => {
                self.set((ptr + self.relative_base) as usize, val);
            }
        }
        self.ip += 1;
    }

    fn set(&mut self, pos: usize, val: i64) {
        self.data[pos] = val
    }

    fn get(&self, posn: usize) -> i64 {
        self.data[posn]
    }

    pub fn run(&mut self) -> i64 {
        loop {
            if let Some(v) = self.step() {
                return v;
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Instr {
    NoneArg(OpNone),
    OneArg(OpOne, Mode),
    TwoArg(OpTwo, (Mode, Mode)),
    ThreeArg(OpThree, (Mode, Mode, Mode)),
}

#[derive(Debug, Clone, Copy)]
enum OpNone {
    Exit,
}

#[derive(Debug, Clone, Copy)]
enum OpOne {
    Input,
    Output,
    AdjustRelativeBase
}

#[derive(Debug, Clone, Copy)]
enum OpTwo {
    JumpIfTrue,
    JumpIfFalse,
}

#[derive(Debug, Clone, Copy)]
enum OpThree {
    Add,
    Mul,
    LessThan,
    Equals,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
    Relative
}

impl TryFrom<u8> for Mode {
    type Error = String;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            other => Err(format!("Bad mode: {}", other)),
        }
    }
}

fn parse_instr(instr: i64) -> Result<Instr, String> {
    let digits = digits(instr as u32);
    let opcode = digits[0] + digits.get(1).cloned().unwrap_or(0) * 10;
    let modes = if digits.len() > 2 { &digits[2..] } else { &[] };
    let ok = match opcode {
        1 => Instr::ThreeArg(OpThree::Add, Mode::three(modes)),
        2 => Instr::ThreeArg(OpThree::Mul, Mode::three(modes)),
        3 => Instr::OneArg(OpOne::Input, Mode::one(modes)),
        4 => Instr::OneArg(OpOne::Output, Mode::one(modes)),
        5 => Instr::TwoArg(OpTwo::JumpIfTrue, Mode::two(modes)),
        6 => Instr::TwoArg(OpTwo::JumpIfFalse, Mode::two(modes)),
        7 => Instr::ThreeArg(OpThree::LessThan, Mode::three(modes)),
        8 => Instr::ThreeArg(OpThree::Equals, Mode::three(modes)),
        9 => Instr::OneArg(OpOne::AdjustRelativeBase, Mode::one(modes)),
        99 => Instr::NoneArg({
            Mode::none(modes);
            OpNone::Exit
        }),
        other => return Err(format!("Bad opcode: {}", other)),
    };
    Ok(ok)
}

fn digits(mut v: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    loop {
        digits.push((v % 10) as u8);
        v = v / 10;
        if v == 0 {
            break;
        }
    }
    digits
}

impl Mode {
    fn get(ix: usize, modes: &[u8]) -> Self {
        modes
            .get(ix)
            .map(|&v| Mode::try_from(v).unwrap())
            .unwrap_or(Mode::Position)
    }

    fn none(modes: &[u8]) -> () {
        assert_eq!(modes.len(), 0);
    }
    fn one(modes: &[u8]) -> Mode {
        assert!(modes.len() <= 1);
        Mode::get(0, modes)
    }
    fn two(modes: &[u8]) -> (Mode, Mode) {
        assert!(modes.len() <= 2);
        let one = Mode::get(0, modes);
        let two = Mode::get(1, modes);
        (one, two)
    }
    fn three(modes: &[u8]) -> (Mode, Mode, Mode) {
        assert!(modes.len() <= 3);
        let one = Mode::get(0, modes);
        let two = Mode::get(1, modes);
        let three = Mode::get(2, modes);
        (one, two, three)
    }
}
