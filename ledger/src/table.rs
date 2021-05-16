mod account;

use crate::test_object::TestObject;

pub trait Table: Default + Sized {
    fn get_table_name(&self) -> String;
    fn get_create_table_query(&self) -> String;
    fn get_drop_table_query(&self) -> String;
    fn get_select_query(&self, where_type: &str) -> String;
    fn get_insert_query(&self) -> String;
    fn get_delete_query(&self, where_type: &str) -> String;
}

pub struct TableContainer {
    table: Vec<Box<dyn std::any::Any>>,
    pub table_name: String,
}

impl TableContainer {
    pub fn new<T>(tbl: Box<T>) -> Self where T: Table + std::any::Any {
        let mut cont = TableContainer {
            table: vec![],
            table_name: tbl.get_table_name().to_string(),
        };
        cont.table.push(tbl);
        return cont;
    }

    pub fn get_table(&self) -> &dyn std::any::Any {
        let a = self.table.get(0).unwrap()
            .downcast_ref::<TestObject>().unwrap();
        return a;
    }
}