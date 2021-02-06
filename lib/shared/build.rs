extern crate prost_build;
// If it must fail > it must panics > no output to concole without panic

fn main() {
    build_pb();

    std::process::Command::new("cargo")
        .arg("fmt")
        .arg("-all")
        .spawn()
        .expect("cargo fmt error");
}

fn build_pb() {
    println!("======================= INSIDE THE BUILD PROJECT =========================");

    let dir = std::fs::read_dir("src/protos/proto/").unwrap();

    let mut vec_protos = vec![];
    for fl in dir {
        let path = format!("{:}", fl.unwrap().path().to_str().unwrap());
        vec_protos.push(path);
    }
    // println!("{:#?}", &vec_protos);

    let mut config = prost_build::Config::default();
    // config.type_attribute(".", "#[derive(Default)]");
    config.out_dir("src/");
    // config.compile_well_known_types();
    // config.type_attribute(".", "#[derive(Debug)]");
    // config.retain_enum_prefix();
    let v = config.compile_protos(&vec_protos, &["src/protos/proto".to_string()]);
    println!("{:?}", v);
    v.unwrap();

    run_format_codes();
}

fn run_format_codes() {
    std::process::Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("cargo fmt");
}
