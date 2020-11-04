// extern crate prost_build;
fn main() {
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
    let v = config.compile_protos(&vec_protos, &["src/pb/proto/".to_string()]);
    println!("{:?}",v);
}
