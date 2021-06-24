use ethereum_types::U256;
use std::borrow::BorrowMut;
use std::cell::Cell;

pub fn max_stack(_pops: &i16, _push: &i16) -> i16 {
    crate::constants::StackLimit + _pops - _push
}

pub fn min_stack(_pops: &i16, _push: &i16) -> i16 {
    _pops.clone()
}

pub fn max_dup_stack(_n: &i16) -> i16 {
    max_stack(_n, &(_n + 1))
}

pub fn min_dup_stack(_n: &i16) -> i16 {
    min_stack(_n, &(_n + 1))
}

pub fn max_swap_stack(_n: &i16) -> i16 {
    max_stack(_n, _n)
}

pub fn min_swap_stack(_n: &i16) -> i16 {
    min_stack(_n, _n)
}

pub struct Stack {
    data: Vec<Cell<U256>>,
    height: u128
}

impl Stack {
    pub fn new(_height: u128) -> Self {
        Stack{data: Vec::new(), height: _height}
    }

    pub fn default() -> Self {
        Stack::new(64)
    }
}

impl Stack {
    pub fn push(&mut self, _data: &U256) -> Result<bool, &str> {
        self.data.push(Cell::new(_data.clone()));
        Ok(true)
    }

    pub fn push_multiple(&mut self, _data: &mut Vec<U256>) {
        for u256v in _data.iter() {
            self.data.push(Cell::new(u256v.clone()));
        }
        // self.data.append(_data)
    }

    pub fn pop(&mut self) -> Result<Cell<U256>, &str> {
        if self.data.len() == 0 { return Err("cannot pop anything"); }
        return Ok(self.data.remove(self.data.len() - 1));
    }

    pub fn peek(&mut self) -> Result<&mut Cell<U256>, U256> {
        if self.data.len() == 0 { return Err(U256::zero()); }
        let l = self.data.len() - 1;
        let m = self.data.get_mut(l);
        return Ok(m.unwrap());
        // return Ok(self.data.get_mut(self.data.len() - 1).unwrap());
    }

    pub fn swap(&mut self, _n: i16) {
        let n = _n as usize;
        let stlen = self.data.len();
        let copy = self.data.get(self.data.len() - n).unwrap().clone();
        self.data[stlen - n] = self.data.get(self.data.len() - 1).unwrap().clone();
        self.data[stlen - 1] = copy;
    }

    pub fn dup(&mut self, _n: i16) {
        let n = _n as usize;
        let target = self.data.len() - _n as usize;
        let element = self.data.get(target).unwrap().clone();
        self.data.push(element);
    }

    pub fn back(&self, _n: i16) -> &Cell<U256> {
        let n = _n as usize;
        self.data.get(self.data.len() - n - 1).unwrap()
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    }

    pub fn print_stack(&self) {
        // let mut revstackdata = Vec::new();
        // for data in &self.data {
        //     revstackdata.push(Cell::new(data.get().clone()));
        // }
        // revstackdata.reverse();
        // for data in &revstackdata {
        //     let vec_data = u256_to_le_u8vec(&data.get());
        //     let mut be_stackdata= vec_data.clone();
        //     be_stackdata.reverse();
        //     print_u8vec(&be_stackdata);
        // }

        for idx in 0..self.data.len() {
            let stackdata = common::u256util::u256_to_le_u8vec(&self.data[idx].get());
            let mut be_stackdata = stackdata.clone();
            // be_stackdata.reverse();
            common::printutil::print_u8vec_rev(&be_stackdata);
        }
    }
}

pub struct ReturnStack {
    data: Vec<u32>
}

impl ReturnStack {
    pub fn new() -> Self {
        ReturnStack{data: Vec::new()}
    }
}

impl ReturnStack {
    pub fn push(&mut self, _d: u32) {
        self.data.push(_d);
    }

    pub fn pop(&mut self) -> Result<u32, bool> {
        if self.data.len() == 0 { return Err(false); }
        let erased_value = self.data.remove(self.data.len() - 1);
        Ok(erased_value)
    }
}