#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

#[macro_use]
extern crate lazy_static;

extern crate config as base_config;

mod config;

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

async fn reject_coverage(c: & mut Client) {
  let no_coverage = c.find(Locator::Id("siNoCoverage-announce")).await;
  match no_coverage {
    Ok(button) => { button.click().await; },
    Err(e) => {}
  }
}


async fn is_logged_in_to_amazon(c: & mut Client)  -> bool {
  let e = c.find(Locator::Id("nav-link-accountList")).await;
  let ok = e.is_ok();
  if ok {
    info!("Logged in!");
  }

  ok 
}

async fn scrape_price(c: & mut Client)-> f32 {
  let e = c.find(Locator::Id("price_inside_buybox")).await;
  let text = e.expect("no price").text().await.unwrap();
  let dollar_string = text.replace("$", "");
  let dollar_string = dollar_string.replace(",", "");

  let float = f32::from_str(&dollar_string).unwrap();
  float
}

async fn is_buy_now_amazon(c: & mut Client)  -> bool {
  let locator = Locator::Id("buyNow");
  let e = c.find(locator).await;
  let ok = e.is_ok();
  if ok {
    info!("Buy it now is available!");
  }

  ok 
}

// async fn exists(c: & mut Client, locator: Locator)  -> bool {
  // let mut e = c.find(locator).await;
  // e.is_ok()
// }



async fn amazon_login(c: & mut Client)  -> Result<(), fantoccini::error::CmdError>{
    // let amazon_sign_in_url = "https://www.amazon.com/sign/s?k=sign+in";
    let amazon_sign_in_url = "https://www.amazon.com/ap/signin?_encoding=UTF8&openid.assoc_handle=usflex&openid.claimed_id=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.identity=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.mode=checkid_setup&openid.ns=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0&openid.ns.pape=http%3A%2F%2Fspecs.openid.net%2Fextensions%2Fpape%2F1.0&openid.pape.max_auth_age=0&openid.return_to=https%3A%2F%2Fwww.amazon.com%2Fgp%2Fcss%2Fhomepage.html%3Fie%3DUTF8%26%252AVersion%252A%3D1%26%252Aentries%252A%3D0";

    c.goto(amazon_sign_in_url).await?;
    // let url = c.current_url().await?;
    //
    let search_form = c.form(Locator::Css("form[name='signIn']")).await?;
    let mut search_input = c.find(Locator::Id("ap_email")).await?;
    let username = JOINEM_CONFIG.username();
    search_input.send_keys(&username).await?;
    search_form.submit().await?;
    // 

    let search_form = c.form(Locator::Css("form[name='signIn']")).await?;
    let mut search_input = c.find(Locator::Id("ap_password")).await?;
    let password = JOINEM_CONFIG.password();
    search_input.send_keys(&password).await?;
    search_form.submit().await?;
    ////

    let e = c.wait_for_find(Locator::Id("nav-link-accountList")).await?;
    println!("Logged in!");
    Ok(())
}


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

fn ignore_subscription() {
    // TODO: Get around subscribe and save
    // //////
    // // match c.find(Locator::Id("buyNew_cbb")).await {
    // match c.find(Locator::Id("buyBoxAccordion")).await {
    //   Ok(element) => {
    //     println!("subscribe and save");
    //     let radio = c.find(Locator::Css(".a-icon-radio-inactive")).await;
    //     radio.unwrap().click();
    //   },
    //   Err(e) => {
    //     println!("not subscribe and save");
    //   }
    // }
}


async fn buy_now(c: & mut Client) -> Result<(), fantoccini::error::CmdError> {
    // let mut buy_now = c.wait_for_find(Locator::Id("buy-now-button"));
    let buy_now = c.wait_for_find(Locator::Id("buyNow"));

    let dur = Duration::from_secs(5);
    match future::timeout(dur, buy_now).await {
      Ok(button) => {
        // println!("yay");
        // Need to wait some time or else will take to shopping cart
        delay_for(Duration::from_millis(5000)).await;

        button.unwrap().click().await;


        // // delay_for(Duration::from_millis(3000)).await;
        // // let form = c.form(Locator::Id("place-order-form")).await;
        // // form.unwrap().submit().await;
        //
        //
        // // let active = c.active_element().await.unwrap();
        // // println!("{:?}", active.element);
        // // enter_frame
        // let frame_id = "turbo-checkout-iframe";
        // let frame = c.wait_for_find(Locator::Id(frame_id)).await;
        // frame.unwrap().enter_frame().await;
        //
        // // // let place_order_id = "turbo-cel-place-order-button";
        // // let place_order_id = "turbo-checkout-pyo-button";
        // let place_order_id = "turbo-checkout-place-order-button";
        // let place_order = c.wait_for_find(Locator::Id(place_order_id)).await;
        // place_order.unwrap().click().await;
        //
        // // wait for confirm
        //
        // info!("You got it dude!");
      },
      Err(e) => {
        println!("boo");
        // c.refresh().await;
      }
    }

    Ok(())
}

async fn confirm_buy_now(c: & mut Client) -> Result<(), fantoccini::error::CmdError> {
  let frame_id = "turbo-checkout-iframe";
  let frame = c.wait_for_find(Locator::Id(frame_id)).await;
  frame.unwrap().enter_frame().await;

  // // let place_order_id = "turbo-cel-place-order-button";
  // let place_order_id = "turbo-checkout-pyo-button";
  let place_order_id = "turbo-checkout-place-order-button";
  let place_order = c.wait_for_find(Locator::Id(place_order_id)).await;
  place_order.unwrap().click().await;

  info!("You got it dude!");

  Ok(())
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
    // static ref PASSWORD: JoinemConfig = JoinemConfig::new()
    // static ref HASHMAP: HashMap<u32, &'static str> = {
        // let mut m = HashMap::new();
        // m.insert(0, "foo");
        // m.insert(1, "bar");
        // m.insert(2, "baz");
        // m
    // };
    // static ref COUNT: usize = HASHMAP.len();
    // static ref NUMBER: u32 = times_two(21);
}

// static PASSWORD: &str = &JoinemConfig::new().password();

async fn check_amazon_item(url: Item) -> Result<(), fantoccini::error::CmdError> {
    let mut c2 = new_client().await.expect("Failed to create new client!");
    c2.goto(&url.2).await?;

    loop {
        if is_buy_now_amazon(& mut c2).await {
          let price = scrape_price(& mut c2).await;
          info!("{} can be bought now!", url.0);
          if price <= url.1 {
            info!("{} Price is good!", url.0);


            buy_now(& mut c2).await;

            // Reject coverage if offered!
            info!("{} Rejecting coverage!", url.0);
            reject_coverage(& mut c2).await;


            delay_for(Duration::from_secs(4)).await;

            // Login if asked
            let current_url = c2.current_url().await?;
            let path = current_url.path();

            // "https://www.amazon.com/ap/signin"
            info!("{} Logging in!", url.0);
            let sign_in_path = "/ap/signin";
            if path.eq(sign_in_path) {
              let search_form = c2.form(Locator::Css("form[name='signIn']")).await?;
              let mut search_input = c2.find(Locator::Id("ap_password")).await?;
              let password = JOINEM_CONFIG.password();
              search_input.send_keys(&password).await?;
              search_form.submit().await?;
            }
              
            // confirm
            info!("{} BUY CONFIRMED!", url.0);
            confirm_buy_now(& mut c2).await;

            delay_for(Duration::from_secs(15)).await;
            // TODO: For now just exit if one is successful
            process::exit(0x0100);
            break;
          } else {
            info!("{} Too Expensive!", url.0);
          }
        } else { 
            info!("{} Not in stock!", url.0);
        }

          // info!("{}, cannot be bought now! Sleeping...", url.0);
          delay_for(Duration::from_secs(15)).await;
          c2.refresh().await;
    }

    c2.close();
    Ok(())

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

