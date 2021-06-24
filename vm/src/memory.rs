use ethereum_types::U256;
use crate::stack::Stack;

pub fn calc_mem_size64(_offset: &U256, _length: &U256) -> (u64, bool) {
    if _length.0[1] == 0
        && _length.0[2] == 0
        && _length.0[3] == 0 {
        return (0, true);
    }
    return calc_mem_size64_with_uint(_offset, _length.0[0]);
}

pub fn calc_mem_size64_with_uint(_offset: &U256, _u64_length: u64) -> (u64, bool) {
    if _u64_length == 0 { return (0, false); }
    let (u64off, mut overflow) = (_offset.0[0], false);
    if _offset.0[1] == 0
        && _offset.0[2] == 0
        && _offset.0[3] == 0 {
        overflow = true;
    }

    if overflow { return (0, true); }

    let val = u64off + _u64_length;
    return (val, val < u64off);
}

pub fn get_data(_data: &Vec<u8>, _start: u64, _size: u64) -> Vec<u8> {
    let length = _data.len() as u64;

    let mut start = _start;
    if start > length { start = length; }

    let mut end = start + _size;
    if end > length { end = length; }

    return common::vecutil::right_pad_bytes(&_data[start as usize..end as usize].to_vec(), _size);
}

pub fn to_u64_size(_size: u64) -> u64 {
    if _size > u64::max_value() - 31 {
        return u64::max_value()/32 + 1;
    }
    return (_size + 31) / 32;
}

pub fn all_zero(_bs: &Vec<u8>) -> bool {
    for b in _bs {
        if b.clone() != 0 { return false; }
    }
    return true;
}

pub fn memory_sha3(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(0).get(), &_stack.back(1).get())
}

pub fn memory_calldata_copy(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(0).get(), &_stack.back(2).get())
}

pub fn memory_returndata_copy(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(0).get(), &_stack.back(2).get())
}

pub fn memory_code_copy(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(0).get(), &_stack.back(2).get())
}

pub fn memory_extcode_copy(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(1).get(), &_stack.back(3).get())
}

pub fn memory_mload(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64_with_uint(&_stack.back(0).get(), 32)
}

pub fn memory_mstore8(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64_with_uint(&_stack.back(0).get(), 1)
}

pub fn memory_mstore(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64_with_uint(&_stack.back(0).get(), 32)
}

pub fn memory_create(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(1).get(), &_stack.back(2).get())
}

pub fn memory_create2(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(1).get(), &_stack.back(2).get())
}

pub fn memory_call(_stack: &Stack) -> (u64, bool) {
    let (x, overflow1) = calc_mem_size64(&_stack.back(5).get(), &_stack.back(6).get());
    if overflow1 { return (0, true); }

    let (y, overflow2) = calc_mem_size64(&_stack.back(3).get(), &_stack.back(4).get());
    if overflow2 { return (0, true); }

    if x > y { return (x, false); }
    return (y, false);
}

pub fn memory_delegate_call(_stack: &Stack) -> (u64, bool) {
    let (x, overflow1) = calc_mem_size64(&_stack.back(4).get(), &_stack.back(5).get());
    if overflow1 { return (0, true); }

    let (y, overflow2) = calc_mem_size64(&_stack.back(2).get(), &_stack.back(3).get());
    if overflow2 { return (0, true); }

    if x > y { return (x, false); }
    return (y, false);
}

pub fn memory_static_call(_stack: &Stack) -> (u64, bool) {
    let (x, overflow1) = calc_mem_size64(&_stack.back(4).get(), &_stack.back(5).get());
    if overflow1 { return (0, true); }

    let (y, overflow2) = calc_mem_size64(&_stack.back(2).get(), &_stack.back(3).get());
    if overflow2 { return (0, true); }

    if x > y { return (x, false); }
    return (y, false);
}

pub fn memory_return(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(0).get(), &_stack.back(1).get())
}

pub fn memory_revert(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(0).get(), &_stack.back(1).get())
}

pub fn memory_log(_stack: &Stack) -> (u64, bool) {
    calc_mem_size64(&_stack.back(0).get(), &_stack.back(1).get())
}

pub struct Memory {
    value: Vec<u8>
}

impl Memory {
    pub fn new() -> Self {
        let mut memval = Vec::new();
        Memory { value: memval }
    }
}

impl Memory {
    pub fn set(&mut self, _offset: u64, _size: u64, _value: &Vec<u8>) {
        if _size > 0 {
            let memsize = self.value.len() as u64;
            if _offset + _size > memsize {
                // panic!("invalid memory: value empty");
                let mut addition = 0;
                if _offset < memsize { addition = memsize - _offset; }
                println!("need more memory! {:x} -> {:x}", self.size(), _offset + _size);
                self.resize(_offset + _size + addition);
                println!("now memory is {:x}", self.size());
            }
            let offset = _offset as usize;
            let offsize = (_offset + _size) as usize;
            self.value[offset..offsize].copy_from_slice(_value);
        }
    }

    pub fn set32(&mut self, _offset: u64, _value: &U256) {
        let memsize = self.size() as u64;
        if _offset + 32 > memsize {
            // println!("memsize({}) is smaller than offset({:x})+0x20", &memsize, &_offset);
            // panic!("invalid memory: value empty");
            // if _offset > memsize {
            //     // if vm tried to store _value(0x01) at _offset(0x80)
            //     // while maximum memsize is 0x60, vm needs to expand memory 0x20 + 32bytes.
            //     let emptyspacesize = _offset - memsize;
            //     println!("empty_space_size:: {:x}", emptyspacesize);
            //     self.resize(emptyspacesize);
            // }
            // self.resize(_offset+32);
            self.resize(_offset+32);
        }
        // self.value.append(&mut u256_to_le_u8vec(_value));
        for idx in _offset.._offset + 32 { self.value[idx as usize] = 0; } // Zero the memory area
        // self.value[_offset as usize..].copy_from_slice(_value.as_byte_slice()); // val.WriteToSlice(m.store[offset:])
        let u8vec = common::u256util::u256_to_le_u8vec(_value);
        self.value[_offset as usize..].copy_from_slice(&u8vec);
    }

    pub fn resize(&mut self, _size: u64) {
        if self.size() < _size as usize {
            // self.value.resize_with(_size as usize - self.size(), Default::default);
            self.value.resize_with(_size as usize, Default::default);
        }
    }

    /// Get returns offset + size as a 'new slice'
    pub fn get_copy(&mut self, _offset: i64, _size: i64) -> Option<Vec<u8>> {
        if _size == 0 { return None; }
        let mut ret = Vec::<u8>::new();
        if self.size() > _offset as usize {
            let begin = _offset as usize;
            let end = (_offset + _size) as usize;
            for i in begin..end { ret.push(0); }
            // println!("memory size: {:02x}", self.value.len());
            // println!("from to {} ~ {}", begin, end);
            if self.value.len() < end { self.value.resize_with(end, Default::default); }
            ret.copy_from_slice(&self.value[begin..end]);
            return Some(ret);
        }
        return Some(ret);
    }

    /// Get returns the offset + size
    // pub fn get_ptr(&self, _offset: i64, _size: i64) -> Option<Vec<u8>> {
    //     if _size == 0 { return None; }
    //     unsafe {
    //         return Some(*self.value[0..2]);
    //     }
    // }

    /// Size returns the length of the backing slice
    pub fn size(&self) -> usize { self.value.len() }

    /// data returns the backing slice
    pub fn data(&self) -> &Vec<u8> { &self.value }

    pub fn print_memory(&self) {
        println!("**memory debug**");
        let mut count = 0;
        for mv in &self.value {
            print!("{:02x} ", mv);
            count += 1; if count == 32 { count = 0; println!(); }
        }
        println!();
        println!("**** **** ****");
    }
}