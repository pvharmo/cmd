use std::collections::HashMap;

use clap::Parser;
use nucleus_rs::storage::{ProvidersMap, ProviderId, ProviderType};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Unique identifier and name of the provider
    #[arg(short, long)]
    id: String,

    /// Type of provider (eg. GoogleDrive, OneDrive, etc.)
    #[arg(short, long)]
    provider: String,

    /// Root for the localfs provider
    #[arg(short, long)]
    root: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let options = nucleus_rs::storage::ProvidersOptions {
        google_api_key: Some(env!("GOOGLE_DRIVE_CLIENT_KEY").to_string()),
        onedrive_api_key: Some(env!("ONEDRIVE_CLIENT_ID").to_string())
    };

    let mut providers = ProvidersMap::new(options).await;

    match args.provider.as_str() {
        "googledrive" => providers.add_google_drive(ProviderId { id: args.id, provider_type: ProviderType::GoogleDrive }, HashMap::new()).await.unwrap(),
        "onedrive" => providers.add_onedrive(ProviderId { id: args.id, provider_type: ProviderType::OneDrive }, None).await.unwrap(),
        "localfs" => providers.add_native_fs(ProviderId { id: args.id, provider_type: ProviderType::NativeFs }, args.root.unwrap()).await.unwrap(),
        _ => panic!("Unknown provider type")
    }
}
