use std::{io, fs};
use fantoccini::{Client, Locator};

// extern crate fs_extra;
// use fs_extra::dir::copy;
// use fs_extra::dir::CopyOptions;
// use log::{info, warn, debug};

use crate::{JOINEM_CONFIG};


pub async fn new_client() -> Result<Client, fantoccini::error::CmdError> {
  // Ok(Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver"))
  // Ok(Client::new("http://localhost:9515").await.expect("failed to connect to WebDriver"))
  //
  
  // let joinem_config2 = JOINEM_CONFIG2.lock().unwrap();

  // let out_dir = JOINEM_CONFIG2.find_or_create_data_folder();
  //
  //
  // let mut num = counter.lock().unwrap();
  // *num += 1;

  let out_dir = JOINEM_CONFIG.find_or_create_data_folder();

  let mut caps = serde_json::map::Map::new();
  let out_dir_arg = format!("--user-data-dir={}", out_dir);
  let mut args = vec!["--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage", &out_dir_arg];
  // args.push("--headless");
  
  let opts = serde_json::json!({
    "args": args,
    "binary":
      if std::path::Path::new("/usr/bin/chromium-browser").exists() {
        // on Ubuntu, it's called chromium-browser
        "/usr/bin/chromium-browser"
      } else if std::path::Path::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome").exists() {
        // macOS
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
      } else if std::path::Path::new("C:/Program Files/Google/Application/chrome.exe").exists() {
        "C:/Program Files/Google/Application/chrome.exe"
      } else {
        // elsewhere, it's just called chromium
        "/usr/bin/chromium"
      }
  });

    caps.insert("goog:chromeOptions".to_string(), opts.clone());

    // let caps = webdriver::capabilities::Capabilities::new()

    Ok(Client::with_capabilities("http://localhost:9515", caps).await.expect("failed to connect to WebDriver"))
}


    // let tab2 = c.new_window(false).await.unwrap();
    //
    // // c.switch_to_window(win2.handle);
    // let windows = c.windows().await.unwrap();
    // // println!("{}", windows.len());
    // // let tab2 = windows.get(1).unwrap().clone();
    // let tab2 = webdriver::common::WebWindow(tab2.handle);
    // c.switch_to_window(tab2).await;

//
