use std::collections::{HashMap, HashSet};
use tokio::sync::mpsc::{channel, Receiver};

// no locking
struct UserSpace {
    contacts: HashSet<String>,
    liked_posts: HashSet<String>,
    commands: Receiver<Commands>,
    last_rpc: u64,

    user_id: u64,
    user_info: Commands, // pb

    contact3: HashMap<String, bool>,
    contacts2: Vec<u64>,
}

enum Commands {
    Rpc,
    InternalRpc,
    Exit,
}
