use std::{io, fs};
use fantoccini::{Client, Locator};

use crate::{JOINEM_CONFIG};

pub async fn new_client() -> Result<Client, fantoccini::error::CmdError> {
  let out_dir = JOINEM_CONFIG.find_or_create_data_folder();
  // let mut caps = JOINEM_CONFIG.caps(&out_dir);
  let mut caps = caps(&out_dir);
  let webdriver_url = JOINEM_CONFIG.webdriver_url.clone().unwrap();
  Ok(Client::with_capabilities(&webdriver_url, caps).await.expect("failed to connect to WebDriver"))
}

pub fn caps(out_dir: &str) -> serde_json::Map<std::string::String, serde_json::Value> {

  let mut caps = serde_json::map::Map::new();
  let out_dir_arg = format!("--user-data-dir={}", out_dir);
  let mut v = vec![out_dir_arg];

  let mut args = JOINEM_CONFIG.args();
  args.append(&mut v);

  let binary = &JOINEM_CONFIG.chrome_bin().clone();

  let opts = serde_json::json!({
    "args": args,
    "binary": binary 
  });

  caps.insert("goog:chromeOptions".to_string(), opts.clone());

  caps
}
