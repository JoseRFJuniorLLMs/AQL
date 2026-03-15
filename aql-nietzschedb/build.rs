fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .compile_protos(
            &["../../NietzscheDB/crates/nietzsche-api/proto/nietzsche.proto"],
            &["../../NietzscheDB/crates/nietzsche-api/proto"],
        )?;
    Ok(())
}
