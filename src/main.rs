extern crate lmdb_rs as lmdb;

use lmdb::{DbFlags, EnvBuilder};

fn main() {
    let env = EnvBuilder::new().open("test-lmdb", 0o777).unwrap();
    let db_handle = env.get_default_db(DbFlags::empty()).unwrap();
    let txn = env.new_transaction().unwrap();
    {
        let db = txn.bind(&db_handle);

        let pairs = vec![("Albert", "Einstein"),
                         ("Joe", "Smith"),
                         ("Jack", "Daniels")];

        for &(name, surname) in pairs.iter() {
            db.set(&surname, &name).unwrap();
        }
    }

    match txn.commit() {
        Err(_) => panic!("failed to commit"),
        Ok(_) => ()
    }
}
