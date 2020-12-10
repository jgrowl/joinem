#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

#[macro_use]
extern crate lazy_static;

extern crate config as base_config;

mod config;
mod amazon;
mod webdriver;

use fantoccini::{Client, Locator};
use tokio::time::delay_for;
use async_std::future;

use std::process;
use std::time::{Duration};
use std::path::Path;

use log::{info, warn, debug};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{JoinemConfig, Item};
use crate::amazon::check_amazon_item;

// use num_bigint::{ToBigInt, RandBigInt};
// use num::traits::ToPrimitive;
use bigdecimal::BigDecimal;
use std::str::FromStr;

// use std::rand::{self, Rng};
use rand::prelude::*;
use std::{io, fs};


extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

lazy_static! {
    static ref JOINEM_CONFIG: JoinemConfig = JoinemConfig::new();
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
      debug!("Starting {:?}", item.0);
      tokio::spawn(async move {
        check_amazon_item(item).await;
      });
    }
}

fn cleanup() {
  // fs::remove_dir_all("/tmp/joinem").unwrap();
  // rm_rf::ensure_removed("/tmp/joinem").expect("couldn't delete");
}


  // delay_for(Duration::from_millis(20000)).await;
  // loop{
    // delay_for(Duration::from_secs(30)).await;
  // }
