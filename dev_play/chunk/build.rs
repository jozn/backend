fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut vec_protos = vec![];
    for fl in std::fs::read_dir("src/proto/").unwrap() {
        let path = format!("{:}", fl.unwrap().path().to_str().unwrap());
        vec_protos.push(path);
    }

    tonic_build::configure().out_dir("./src/proto_gen").compile(
        &vec_protos,
        &["src/proto".to_string()],
    );

    // tonic_build::compile_protos("src/proto/helloworld.proto")?;

    Ok(())
}
