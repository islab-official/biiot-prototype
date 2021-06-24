pub mod body;
pub mod header;
pub mod request;
pub mod response;
pub mod server;
pub mod status;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
