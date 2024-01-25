use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protocol/api_gateway.proto")?;
    Ok(())
}
