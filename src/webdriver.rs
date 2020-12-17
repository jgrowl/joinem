use std::{io, fs};
use fantoccini::{Client, Locator};

use crate::{JOINEM_CONFIG};

pub async fn new_client() -> Result<Client, fantoccini::error::CmdError> {
  let out_dir = JOINEM_CONFIG.find_or_create_data_folder();
  let mut caps = JOINEM_CONFIG.caps(&out_dir);
  let webdriver_url = JOINEM_CONFIG.webdriver_url.clone().unwrap();
  Ok(Client::with_capabilities(&webdriver_url, caps).await.expect("failed to connect to WebDriver"))
}
