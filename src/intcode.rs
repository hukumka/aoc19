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
        let out_addr = $self.data[$self.ip] as usize;
        $self.ip += 1;
        let $out = &mut $self.data[out_addr];
    };

    (#params $self: expr, $param: expr, (|| => $out: ident)) => {
        let out_addr = $self.data[$self.ip] as usize;
        $self.ip += 1;
        let $out = &mut $self.data[out_addr];
    };

    (#params $self: expr, $param: expr, (|$($var: ident),*|)) => {
        $(let $var = $self.inp(&mut $param);)*
    };

    (#params $self: expr, $param: expr, (||)) => {}
}

pub struct IntCode {
    data: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    ip: usize,
}

impl IntCode {
    pub fn new(data: Vec<i32>) -> Self {
        Self {
            data,
            input: VecDeque::new(),
            output: VecDeque::new(),
            ip: 0,
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
        }

        true
    }

    pub fn input(&mut self) -> &mut VecDeque<i32> {
        &mut self.input
    }

    pub fn output(&mut self) -> &mut VecDeque<i32> {
        &mut self.output
    }

    fn inp(&mut self, params: &mut i32) -> i32 {
        let mode = *params % 10;
        *params /= 10;
        let value = match mode {
            0 => self.data[self.data[self.ip] as usize],
            1 => self.data[self.ip],
            _ => panic!(),
        };
        self.ip += 1;
        value
    }
}
