pub fn print_u8vec(_data: &Vec<u8>) {
    for idx in 0.._data.len() {
        print!("{:02x} ", _data[idx]);
    } println!();
}

pub fn print_u8vec_rev(_data: &Vec<u8>) {
    for idx in (0.._data.len()).rev() {
        print!("{:02x} ", _data[idx]);
    } println!();
}