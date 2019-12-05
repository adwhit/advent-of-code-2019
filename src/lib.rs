use std::convert::TryFrom;
use std::io::{self, BufRead, Write};

#[derive(Debug, Clone)]
pub struct Machine {
    data: Vec<i32>,
    ip: usize,
}

impl Machine {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data, ip: 0 }
    }

    pub fn init(mut data: Vec<i32>, v1: i32, v2: i32) -> Self {
        data[1] = v1;
        data[2] = v2;
        Self::new(data)
    }

    pub fn from_file(path: &str) -> Self {
        let data = std::fs::read_to_string(path)
            .unwrap()
            .trim()
            .split(',')
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        Self::new(data)
    }

    pub fn init_from_file(path: &str, v1: i32, v2: i32) -> Self {
        let machine = Self::from_file(path);
        Machine::init(machine.data, v1, v2)
    }

    pub fn state(&self) -> &[i32] {
        &self.data
    }

    pub fn step(&mut self) -> Option<i32> {
        match parse_instr(self.fetch(Mode::Immediate)).unwrap() {
            Instr::NoneArg(OpNone::Exit) => return Some(self.get(0)),
            Instr::OneArg(arg, mode1) => match arg {
                OpOne::Input => {
                    let num = loop {
                        print!("Input > ");
                        io::stdout().flush().unwrap();
                        let stdin = io::stdin();
                        let line = stdin
                            .lock()
                            .lines()
                            .next()
                            .expect("No line")
                            .expect("Bad read");
                        match line.trim().parse::<i32>() {
                            Ok(num) => break num,
                            Err(_) => println!("Bad input"),
                        }
                    };
                    self.fetch_and_set(mode1, num);
                }
                OpOne::Output => {
                    let out = self.fetch(mode1);
                    println!("{}", out);
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

    fn fetch(&mut self, mode: Mode) -> i32 {
        let param = self.data[self.ip];
        self.ip += 1;
        match mode {
            Mode::Immediate => param,
            Mode::Position => self.data[param as usize],
        }
    }

    fn fetch_and_set(&mut self, mode: Mode, val: i32) {
        assert_eq!(mode, Mode::Position);
        let ptr = self.data[self.ip];
        self.set(ptr as usize, val);
        self.ip += 1;
    }

    fn set(&mut self, pos: usize, val: i32) {
        self.data[pos] = val
    }

    fn get(&self, posn: usize) -> i32 {
        self.data[posn]
    }

    pub fn run(&mut self) -> i32 {
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
}

impl TryFrom<u8> for Mode {
    type Error = String;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            other => Err(format!("Bad mode: {}", other)),
        }
    }
}

fn parse_instr(instr: i32) -> Result<Instr, String> {
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
