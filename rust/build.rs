fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("rust/generated")
        .compile(
            &[
                "./node/api.proto",
                "./node/rpc/block.proto",
                "./node/rpc/node_admin.proto",
                "./node/rpc/node_wallet_manager.proto",
            ],
            &["./"],
        )?;
    Ok(())
}
