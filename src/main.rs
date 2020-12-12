#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

#[macro_use]
extern crate lazy_static;

extern crate config as base_config;

mod config;
mod amazon;
mod webdriver;
mod util;

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

use std::sync::{Arc, Mutex, MutexGuard};

lazy_static! {
	static ref JOINEM_CONFIG: JoinemConfig = {
		let config = JoinemConfig::new();
		config.unwrap()
	};

    // WARNING: Using a global variable here to keep track of which chrome
    // data dirs are currently in use. This is necessary because if we 
    // don't we will have to copy the folder every time and then clean it up
    // Using this global will allow us to just reuse the same folders
    // between runs.
    //
    // WARNING: This will let cache become stale so may need to provide
    // a way to clean cache. 
    //
    // WARNING: This will also cause problems if you ever want to reuse 
    // a data dir while the program is already runnnig. This will have to 
    // be managed.
    // static ref DATA_DIRS: Arc<Mutex<Vec<String>>>
      // = Arc::new(Mutex::new(Vec::new()));
    static ref DATA_DIRS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub fn get_data_dirs<'a>() -> MutexGuard<'a, Vec<String>> {
        DATA_DIRS.lock().unwrap()
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
  let mut bots = run_bots().await;


  while running.load(Ordering::SeqCst) { }
  println!("\nShutting down joinem!");

  cleanup(bots);


  Ok(())
}

async fn run_bots() -> Vec<BotType> {
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
    let items = JOINEM_CONFIG.items.clone();

    let mut spawns = vec![]; 
    for item in items.into_iter() {
      let the_item = item.clone();
      debug!("Starting {:?}", item.name);
      let spawn = tokio::spawn(async move {
        check_amazon_item(item.clone()).await;
      });
      spawns.push((the_item, spawn));

      // We have to wait because we are using a global variable in a
      // multi-threaded app. If we don't do this then another thread 
      // grab the chrome data directory before the thread that
      // created it has a chance to register the name
      //
      // WARNING: Touching the data_dirs global will be dangerous!
      //
      delay_for(Duration::from_secs(1)).await
    }

    return spawns;
}

type BotType = (Item, tokio::task::JoinHandle<()>);
fn cleanup(bots: Vec<BotType>) {
  for bot in bots {
    println!("Doin somethin");
    // println!("Doin somethin for {}", bot.name);
  }
  // fs::remove_dir_all("/tmp/joinem").unwrap();
  // rm_rf::ensure_removed("/tmp/joinem").expect("couldn't delete");
}


  // delay_for(Duration::from_millis(20000)).await;
  // loop{
    // delay_for(Duration::from_secs(30)).await;
  // }
