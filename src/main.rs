use lib;
use biiot_prototype::Biiot;


fn main() {
    let biiot = Biiot::new();
    let result = biiot.check_gpio_link();
    if result.is_none() { panic!("some GPIO connections are missing") }
    biiot.run_system();
}
