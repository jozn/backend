use rocksdb::{ColumnFamilyDescriptor, Direction, IteratorMode, Options, WriteBatch, DB};
use std::str;
fn cf() {
    let path = "./rock2.db";
    let mut cf_opts = Options::default();
    cf_opts.set_max_write_buffer_number(16);
    let cf = ColumnFamilyDescriptor::new("cf1", cf_opts);

    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    {
        let db = DB::open_cf_descriptors(&db_opts, path, vec![cf]).unwrap();
    }
    // let _ = DB::destroy(&db_opts, path);
}

fn cf_loop() {
    let path = "./rock2.db";
    let mut cf_opts = Options::default();
    cf_opts.set_max_write_buffer_number(16);
    let cf = ColumnFamilyDescriptor::new("cf1", cf_opts);

    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    {
        let db = DB::open_cf_descriptors(&db_opts, path, vec![cf]).unwrap();
    }
    // let _ = DB::destroy(&db_opts, path);
}

fn simple() {
    // NB: db is automatically closed at end of lifetime
    let path = "./rokcs1.db";
    {
        let db = DB::open_default(path).unwrap();
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }
    // let _ = DB::destroy(&Options::default(), path);
}

fn simple_loop() {
    // NB: db is automatically closed at end of lifetime
    let path = "./rokcs3.db";
    {
        let db = DB::open_default(path).unwrap();
        for i in 0..1000_000 {
            let key = format!("my key new new nwe {}", i);
            let val = format!("my value {}", i);
            if (i % 1000) == 0 {
                println!("{}", i);
            }
            db.put(key, val).unwrap();
        }

        match db.get(b"my key 99") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
    }
    // let _ = DB::destroy(&Options::default(), path);
}

fn simple_loop_batch() {
    // NB: db is automatically closed at end of lifetime
    let path = "./rokcs5.db";
    {
        let db = DB::open_default(path).unwrap();

        for i in 0..100_00 {
            let mut batch = WriteBatch::default();

            for j in 0..1 {
                let key = format!("my batch {} {}", i, j);
                let val = format!("my value {}", i);
                batch.put(key, val);
            }
            if (i % 10) == 0 {
                println!("batch {}", i * 100);
            }
            db.write(batch); // Atomically commits the batch
        }

        match db.get(b"my key 99") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
    }
    // let _ = DB::destroy(&Options::default(), path);
}

fn simple_loop_batch_read() {
    // NB: db is automatically closed at end of lifetime
    let path = "./rokcs6.db";
    DB::destroy(&Options::default(), path);

    {
        let db = DB::open_default(path).unwrap();
        for i in 0..100 {
            let key = format!("my read {}", i);
            let val = format!("my value {}", i);
            if (i % 1000) == 0 {
                println!("{}", i);
            }
            db.put(key, val).unwrap();
        }

        match db.get(b"my key 99") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
    }

    {
        let db = DB::open_default(path).unwrap();
        let mut iter = db.iterator(IteratorMode::Start); // Always iterates forward
        for (key, value) in iter {
            println!("Start {:?} {:?}", String::from_utf8(key.into()), value);
        }
        iter = db.iterator(IteratorMode::End); // Always iterates backward
        for (key, value) in iter {
            println!("End {:?} {:?}", String::from_utf8(key.into()), value);
        }
        iter = db.iterator(IteratorMode::From(b"my read 50", Direction::Forward)); // From a key in Direction::{forward,reverse}
        for (key, value) in iter {
            println!("Forward50  {:?} {:?}", String::from_utf8(key.into()), value);
        }

        // You can seek with an existing Iterator instance, too
        iter = db.iterator(IteratorMode::Start);
        iter.set_mode(IteratorMode::From(b"my read 50", Direction::Reverse));
        for (key, value) in iter {
            println!("Reverse50 {:?} {:?}", String::from_utf8(key.into()), value);
        }
    }
    // let _ = DB::destroy(&Options::default(), path);
}

fn main() {
    // simple_loop();
    // simple_loop_batch();
    simple_loop_batch_read();
}
