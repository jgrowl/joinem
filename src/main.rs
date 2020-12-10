#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

#[macro_use]
extern crate lazy_static;

extern crate config as base_config;

mod config;
mod amazon;

use fantoccini::{Client, Locator};
use tokio::time::delay_for;
use async_std::future;

use std::process;
use std::time::{Duration};
use std::path::Path;

use log::{info, warn};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::config::{JoinemConfig, Item};
use crate::amazon::check_amazon_item;

// use num_bigint::{ToBigInt, RandBigInt};
// use num::traits::ToPrimitive;
use bigdecimal::BigDecimal;
use std::str::FromStr;

// use std::rand::{self, Rng};
use rand::prelude::*;
use std::{io, fs};

extern crate fs_extra;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;

extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

async fn new_client() -> Result<Client, fantoccini::error::CmdError> {
  // Ok(Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver"))
  // Ok(Client::new("http://localhost:9515").await.expect("failed to connect to WebDriver"))
  //
  
  let out_dir_base = "/tmp/joinem";
  // std::fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");

  let out_dir = format!("{}/{}", out_dir_base, random_string());

  // fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");
  //
  // THIS ONE
  fs::create_dir_all(&out_dir).expect("Failed to create directory!");
  //
  // println!("outdir: {}", out_dir);
  // copy_dir_all("~/Library/Caches/Google/Chrome/Default", out_dir);

  let options = CopyOptions::new(); //Initialize default values for CopyOptions
// options.mirror_copy = true; // To mirror copy the whole structure of the source directory

// copy source/dir1 to target/dir1
// let default = "~/Library/Caches/Google/Chrome/Default";
let default = JOINEM_CONFIG.chrome_user_data();
// println!("{}", default);
// let default = "/Users/jon/Library/Caches/Google/Chrome";
// copy(default, &out_dir, &options).expect("uho");
//

  // this actually copies
  copy_dir_all(default, &out_dir);


  // use std::os::unix::fs;
  // fs::symlink(default, &out_dir).unwrap();


  let mut caps = serde_json::map::Map::new();
  let opts = serde_json::json!({
    // "args": ["--headless", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"],
    "args": ["--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage", format!("--user-data-dir={}", out_dir)],
    // "args": ["--headless", "--disable-gpu", "--no-sandbox", "--disable-dev-shm-usage"],
    "binary":
      if std::path::Path::new("/usr/bin/chromium-browser").exists() {
        // on Ubuntu, it's called chromium-browser
        "/usr/bin/chromium-browser"
      } else if std::path::Path::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome").exists() {
        // macOS
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
      } else {
        // elsewhere, it's just called chromium
        "/usr/bin/chromium"
      }
  });

    caps.insert("goog:chromeOptions".to_string(), opts.clone());

    // let caps = webdriver::capabilities::Capabilities::new()

    Ok(Client::with_capabilities("http://localhost:9515", caps).await.expect("failed to connect to WebDriver"))
}

fn random_string() -> String {
      let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .collect();

     rand_string
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


lazy_static! {
    static ref JOINEM_CONFIG: JoinemConfig = JoinemConfig::new();
}

fn run_bots() {
    // let mut c = new_client().await.expect("Failed to create new client!");
    // if !is_logged_in_to_amazon(& mut c).await {
    //   info!("Not logged into Amazon.");
    //   info!("Attempting to login to Amazon!");
    //   amazon_login(& mut c).await;
    // } else {
    //   info!("Already logged in to Amazon.");
    // }
    //
    // c.close().await;

    // let url = "https://www.amazon.com/AMD-Ryzen-5950X-32-Thread-Processor/dp/B0815Y8J9N";
    // check_amazon_item(url).await;
    let items = JOINEM_CONFIG.items();

    for item in items.into_iter() {
      tokio::spawn(async move {
        check_amazon_item(item).await;
      });
    }
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
  env_logger::init();

  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();

  ctrlc::set_handler(move || {
    r.store(false, Ordering::SeqCst);
  }).expect("Error setting Ctrl-C handler");

  println!("Waiting for Ctrl-C...");
  run_bots();

  while running.load(Ordering::SeqCst) { }
  println!("\nShutting down joinem!");

  cleanup();

  // delay_for(Duration::from_millis(20000)).await;
  // loop{
    // delay_for(Duration::from_secs(30)).await;
  // }

  Ok(())
}

fn cleanup() {
  // fs::remove_dir_all("/tmp/joinem").unwrap();
  // rm_rf::ensure_removed("/tmp/joinem").expect("couldn't delete");
}

