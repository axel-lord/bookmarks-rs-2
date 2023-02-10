use bookmark_data::FileData;
use bytesize::ByteSize;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let data = FileData::load(cli.file).await?;
    let size = ByteSize::b(data.storage_size().try_into()?);

    println!("size of file contents: {size}");

    Ok(())
}
