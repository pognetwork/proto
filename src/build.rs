fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("src/generated")
        .compile(&["./node/api.proto", "./node/rpc.proto"], &["./"])?;
    Ok(())
}
