use clap::Parser;

use quick_xml;

use serde::Deserialize;

use anyhow::Error;

use tokio::{
  process::Command,
  fs::{remove_file, File},
  io::copy,
};

use std::path::Path;

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

#[derive(Deserialize)]
struct ModLoaderVersion {
  metadata: XMLMetadata
}

#[derive(Deserialize)]
#[warn(dead_code)]
struct XMLMetadata {
  #[serde(rename = "groupId")]
  group_id: String,
  artifactId: String,
  versioning: Versions
}

#[derive(Deserialize)]
struct Versions {
  #[serde(rename = "latest")]
  latest_version: String,
  release: String,
  versions: Vec<String>,
  #[serde(rename = "lastUpdated")]
  last_updated: String
}

#[tokio::main]
async fn main() {
  let args = ForgerParser::parse();
}

async fn install_fabric(server: bool, version: String) -> Result<(), Error> {
  let serverstring = if server {
    "server"
  } else {
    "client"
  };
  let response = reqwest::get("https://maven.fabricmc.net/net/fabricmc/fabric-installer/maven-metadata.xml").await?.text().await?;
  let version: ModLoaderVersion = quick_xml::de::from_str(&response)?;
  let version = version.metadata.versioning.latest_version;
  drop(response);
  let filename = format!("fabric-installer-{}{}.jar", version, serverstring);
  if Path::new(&filename).exists() {
    remove_file(filename.clone()).await?;
  }
  let response = reqwest::get(format!("https://maven.fabricmc.net/net/fabricmc/fabric-installer/{}/{}", version, filename)).await?;
  let mut file = File::create(filename.clone()).await?;
  let response_content = response.bytes().await?;
  let mut bresponse_content = response_content.as_ref();
  copy(&mut bresponse_content, &mut file).await?;
  drop(bresponse_content);
  Command::new("java").arg("-jar").arg(filename).args(&[serverstring, "-mcversion"]).arg(version).arg("-downloadMinecraft").spawn()?;
  Ok(())
}
