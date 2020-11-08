extern crate prost_build;
// If it must fail > it must panics > no output to concole without panic

fn main() {
    println!("======================= INSIDE THE BUILD PROJECT =========================");

    let dir = std::fs::read_dir("src/protos/proto/").unwrap();

    let mut vec_protos = vec![];
    for fl in dir {
        let path = format!("{:}",fl.unwrap().path().to_str().unwrap());
        vec_protos.push(path);
    }
    // println!("{:#?}", &vec_protos);

    let mut config = prost_build::Config::default();
    config.out_dir("src/");
    // config.compile_well_known_types();
    // config.type_attribute(".", "#[derive(Debug)]");
    // config.retain_enum_prefix();
    let v = config.compile_protos(&vec_protos, &["src/protos/proto".to_string()]);
    v.unwrap();
}

fn main_bk() {
    println!("======================= INSIDE THE BUILD PROJECT =========================");

    let protos_files = [
        "enums.proto",
        "global.proto",
        "rpc_account.proto",
        "rpc_auth.proto",
        "rpc_channel.proto",
        "rpc_chat.proto",
        "rpc_direct.proto",
        "rpc_general.proto",
        "rpc_group.proto",
        "rpc_sample.proto",
        "rpc_social.proto",
        "rpc_upload.proto",
        "store.proto",
        "sys.proto",
        "views.proto",
    ];

    let mut vec_protos = vec![];
    for p in &protos_files {
        vec_protos.push(format!("src/protos/proto/{}",p));
    }

    let mut config = prost_build::Config::default();
    config.out_dir("src/");
    config.compile_well_known_types();
    config.retain_enum_prefix();
    let v = config.compile_protos(&vec_protos, &["src/protos/proto".to_string()]);
    v.unwrap();
}
