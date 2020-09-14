use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use anyhow::bail;
use log::{error, info, warn};
use serde_json::Value;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "echovr-rs")]
pub struct Options {
    #[structopt(
        short,
        long,
        default_value = "127.0.0.1",
        help = "IP address where Echo VR can be reached; if using Quest, this is your Quests's WiFi IP address"
    )]
    pub address: String,
}

async fn get_frame(address: impl AsRef<str>) -> anyhow::Result<HashMap<String, Value>> {
    let url = format!("http://{}:6721/session", address.as_ref());
    info!("{}", url);

    let resp = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("HTTP error: {}", e.to_string());
            bail!(e);
        }
    };

    let resp = match resp.json().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Response could not be decoded as JSON:\n{}", e.to_string());
            bail!(e);
        }
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let options = Options::from_args();

    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    loop {
        match get_frame(&options.address).await {
            Ok(response) => {
                info!("{:?}", response);
            }
            Err(_) => {
                warn!("Could not access API, trying again in 3 seconds");
                thread::sleep(Duration::from_secs(3))
            }
        }
    }
}
