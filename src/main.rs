use std::{sync::Arc, thread::sleep, time::Duration};

use bson::doc;
use mongo_driver::client::{ClientPool, Uri};

fn main() {
    dbops();
    println!("DB ops finished");
    sleep(Duration::from_secs(300));
}

fn dbops() {
    let uri = Uri::new("mongodb://localhost:27017/").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    let client = pool.pop();
    let db = client.get_database("mongo_driver_mem_leak");
    let collection = db.get_collection("MemTest");

    let array: Vec<u32> = (0..10000).collect();
    collection.remove(&doc! {}, None).unwrap();
    collection
        .insert(&doc! { "doc_data": array }, None)
        .unwrap();

    for _ in 0..100000 {
        // > Keeps consuming more and more
        let _ = collection
            .command_simple(doc! { "find": "MemTest" }, None)
            .unwrap();

        // > Uses 2.4MB and never increases
        // let _ = collection.find(&doc! {}, None);
        sleep(Duration::from_millis(1));
    }
}
