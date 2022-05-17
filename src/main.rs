use serde::Deserialize;
use clap::Parser;

/// Download fabric, quilt server or client with cli.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct ForgerParser {
  /// Modloader that will be download.
  #[clap(long)]
  modloader: String,

  /// Minecraft version that will be download.
  #[clap(long)]
  version: String,

  /// Server or client version?
  #[clap(long)]
  server: Option<bool>
}

fn main() {
  let args = ForgerParser::parse();
}

// https://maven.fabricmc.net/net/fabricmc/fabric-installer/maven-metadata.xml
fn install_fabric() {

}
