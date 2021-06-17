fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().out_dir("./src/proto_gen").compile(
        &["helloworld.proto".to_string()],
        &["src/proto".to_string()],
    );

    // tonic_build::compile_protos("src/proto/helloworld.proto")?;

    Ok(())
}
