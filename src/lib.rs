pub struct Machine {
    data: Vec<usize>,
    ip: usize
}

impl Machine {
    pub fn new(data: Vec<usize>) -> Self {
        Self {
            data,
            ip: 0
        }
    }

    pub fn init(mut data: Vec<usize>, v1: usize, v2: usize) -> Self {
        data[1] = v1;
        data[2] = v2;
        Self {
            data,
            ip: 0
        }
    }

    pub fn state(&self) -> &[usize] {
        &self.data
    }

    fn fetch(&mut self) -> usize {
        let rtn = self.data[self.ip];
        self.ip += 1;
        rtn
    }

    fn fetch_deref(&mut self) -> usize {
        let ptr = self.data[self.ip];
        self.ip += 1;
        self.data[ptr]
    }

    fn set(&mut self, pos: usize, val: usize) {
        self.data[pos] = val
    }

    fn get(&self, posn: usize) -> usize {
        self.data[posn]
    }

    pub fn run(&mut self) -> usize {
        loop {
            match self.fetch() {
                1 => {
                    let v1 = self.fetch_deref();
                    let v2 = self.fetch_deref();
                    let target = self.fetch();
                    self.set(target, v1 + v2)
                }
                2 => {
                    let v1 = self.fetch_deref();
                    let v2 = self.fetch_deref();
                    let target = self.fetch();
                    self.set(target, v1 * v2)
                }
                99 => return self.get(0),
                other => panic!("Bad opcode at posn {}: {}", self.ip - 1, other),
            }
        }
    }
}
