fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("src/proto/hw1.tasks.proto")?;
    Ok(())
}
