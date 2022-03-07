fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .out_dir("rust/generated")
        .compile(
            &[
                "./node/api.proto",
                "./node/p2p.proto",
                "./node/rpc/lattice.proto",
                "./node/rpc/node_admin.proto",
                "./node/rpc/node_wallet_manager.proto",
                "./node/rpc/node_user.proto",
            ],
            &["./"],
        )?;
    Ok(())
}
