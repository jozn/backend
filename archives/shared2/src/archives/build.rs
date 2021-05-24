fn main_bk() {
    // prost_build::compile_protos(&["src/items.proto"],
    //                             &["src/"]);

    let arr = &[
        "src/pb/proto/enums.proto",
        "src/pb/proto/global.proto",
        "src/pb/proto/rpc_auth.proto",
        "src/pb/proto/rpc_general.proto",
        "src/pb/proto/store.proto",
        "src/pb/proto/sys.proto",
        "src/pb/proto/views.proto",
    ];
    // prost_build::compile_protos(arr, &["src/pb/proto/"]).unwrap();

    let mut config = prost_build::Config::default();
    config.out_dir("src/pb/ps/");
    config.compile_well_known_types();
    config.retain_enum_prefix();
    config.compile_protos(arr, &["src/pb/proto/"]).unwrap();
}
extern crate pb_rs;

use pb_rs::types::{Config, FileDescriptor};
use std::env;
use std::path::{Path, PathBuf};

fn main4() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "src/pb/prost/".to_string();
    let out_file = Path::new(&out_dir).join("hello.rs");

    let config = Config {
        // in_file: PathBuf::from("protos/Hello.proto"),
        in_file: PathBuf::from("src/proto/pb_views.proto"),
        out_file,
        single_module: false,
        import_search_path: vec![PathBuf::from("src/proto/")],
        no_output: false,
        error_cycle: false,           // may change a required field to an optional
        headers: false,               // do not generate headers
        dont_use_cow: false,          // Don't use Cow<_,_> for Strings and Bytes
        custom_struct_derive: vec![], // Nothing
        custom_repr: None,
        custom_rpc_generator: Box::new(|_, _| Ok(())),
        custom_includes: vec![],
        owned: true,
        nostd: false,
        hashbrown: false,
    };

    FileDescriptor::write_proto(&config).unwrap();
}
