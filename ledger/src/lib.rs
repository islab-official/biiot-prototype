pub mod account;
pub mod transaction;
mod pool;
mod table;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
