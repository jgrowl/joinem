use std::{io, fs};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::Path;
use fantoccini::{Client, Locator};

extern crate fs_extra;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use log::{info, warn, debug};

use crate::JOINEM_CONFIG;

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

pub async fn new_client() -> Result<Client, fantoccini::error::CmdError> {
  // Ok(Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver"))
  // Ok(Client::new("http://localhost:9515").await.expect("failed to connect to WebDriver"))
  //
  
  let out_dir_base = JOINEM_CONFIG.data();
  // std::fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");

  let out_dir = format!("{}/{}", out_dir_base, random_string());

  // fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");
  //
  fs::create_dir_all(&out_dir).expect("Failed to create directory!");
  //
  // println!("outdir: {}", out_dir);
  // copy_dir_all("~/Library/Caches/Google/Chrome/Default", out_dir);

  let options = CopyOptions::new(); //Initialize default values for CopyOptions
// options.mirror_copy = true; // To mirror copy the whole structure of the source directory

// copy source/dir1 to target/dir1
// let default = "~/Library/Caches/Google/Chrome/Default";
let default = JOINEM_CONFIG.chrome_user_data();
debug!("Chrome user_data path set to {}", &default);
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
