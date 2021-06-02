pub mod ledger;
pub mod account;
pub mod transaction;
pub mod pool;
mod table;
mod sql_util;
mod constant;
mod dirty_state;

#[cfg(test)]
mod tests {
    use crate::ledger::Ledger;

    #[test]
    fn create_ledger() {
        let mut ledger = Ledger::new();
        ledger.initialize();
    }

    #[test]
    fn store_data() {
        let mut ledger = Ledger::new();
    }
}
