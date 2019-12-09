use std::collections::VecDeque;

macro_rules! optcode{
    (   ($self: expr, $match_code: expr, $param: ident)
        $($code: expr => $params: tt $tok: tt,)*
    ) => {
        $self.ip += 1;
        match $match_code{
            $($code => {
                optcode!{#params $self, $param, $params}
                $tok
            },)*
            x => panic!("Unknown optcode {};", x)
        }
    };

    (#params $self: expr, $param: expr, (|$($var: ident),*| => $out: ident)) => {
        $(let $var = $self.inp(&mut $param);)*
        let addr = $self.get_addr(&mut $param);
        $self.prep_addr(addr);
        let $out = &mut $self.data[addr as usize];
    };

    (#params $self: expr, $param: expr, (|| => $out: ident)) => {
        let addr = $self.get_addr(&mut $param);
        $self.prep_addr(addr);
        let $out = &mut $self.data[addr as usize];
    };

    (#params $self: expr, $param: expr, (|$($var: ident),*|)) => {
        $(let $var = $self.inp(&mut $param);)*
    };

    (#params $self: expr, $param: expr, (||)) => {}
}

pub struct IntCode {
    data: Vec<i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    ip: usize,
    relative_base: i64,
}

impl IntCode {
    pub fn new(data: Vec<i64>) -> Self {
        Self {
            data,
            input: VecDeque::new(),
            output: VecDeque::new(),
            ip: 0,
            relative_base: 0,
        }
    }

    fn get_data(&self, addr: i64) -> i64 {
        if addr as usize >= self.data.len() {
            0
        } else {
            self.data[addr as usize]
        }
    }

    fn prep_addr(&mut self, addr: i64) {
        if addr as usize >= self.data.len() {
            self.data.resize(addr as usize + 1, 0);
        }
    }

    /// returns true if still running, false if complete
    pub fn step(&mut self) -> bool {
        let code = self.data[self.ip];
        let opt = code % 100;
        let mut param = code / 100;

        optcode! { (self, opt, param)
            // optcode takes two arguments (a, b)
            // and stores result (c)
            1 => (|a, b| => c){
                *c = a + b;
            },
            2 => (|a, b| => c){
                *c = a * b;
            },
            99 => (||){
                return false;
            },
            3 => (|| => a){
                *a = self.input.pop_front().unwrap();
            },
            4 => (|a|){
                self.output.push_back(a);
            },
            5 => (|a, b|){
                if a != 0{
                    self.ip = b as usize;
                }
            },
            6 => (|a, b|){
                if a == 0{
                    self.ip = b as usize;
                }
            },
            7 => (|a, b| => c){
                if a < b{
                    *c = 1;
                }else{
                    *c = 0;
                }
            },
            8 => (|a, b| => c){
                if a == b{
                    *c = 1;
                }else{
                    *c = 0;
                }
            },
            9 => (|x|){
                self.relative_base += x;
            },
        }

        true
    }

    pub fn input(&mut self) -> &mut VecDeque<i64> {
        &mut self.input
    }

    pub fn output(&mut self) -> &mut VecDeque<i64> {
        &mut self.output
    }

    fn get_addr(&mut self, params: &mut i64) -> i64 {
        let mode = *params % 10;
        *params /= 10;
        let res = match mode {
            0 => self.data[self.ip],
            1 => self.ip as i64,
            2 => self.data[self.ip] + self.relative_base,
            _ => panic!(),
        };
        self.ip += 1;
        res
    }

    fn inp(&mut self, params: &mut i64) -> i64 {
        let addr = self.get_addr(params);
        self.get_data(addr)
    }
}
