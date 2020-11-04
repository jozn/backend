
fn mem_size(){

    let s = std::mem::size_of::<pb::store::Message>();
    println!("size: {}", s );

    let s = std::mem::size_of::<pb::store::MessageCount>();
    println!("size msg count: {}", s );

    let s = std::mem::size_of::<pb::ActionView>();
    println!("size compact: {}", s );

}
