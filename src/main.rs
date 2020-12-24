#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

#[macro_use]
extern crate lazy_static;

extern crate config as base_config;

mod types;
mod config;
mod amazon;
mod newegg;
mod webdriver;
mod util;

use fantoccini::{Client, Locator};
use tokio::time::delay_for;
use async_std::future;

use std::process;
use std::time::{Duration};
use std::path::Path;

use log::{info, warn, debug, error};
use log4rs;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{JoinemConfig, Item};
use crate::types::ElementResult;
use crate::amazon::check_amazon_item;
use crate::newegg::{Bot as NeweggBot};

use crate::webdriver::new_client;
use crate::newegg::elements::NeweggElements;

// use num_bigint::{ToBigInt, RandBigInt};
// use num::traits::ToPrimitive;
use bigdecimal::BigDecimal;
use std::str::FromStr;

// use std::rand::{self, Rng};
use rand::prelude::*;
use std::{io, fs};

// use crate::types::Action::{Stay, Wait, Click, End};
use crate::types::Action::*;
use crate::types::Action;


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
  static ref DATA_DIRS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub fn get_data_dirs<'a>() -> MutexGuard<'a, Vec<String>> {
  DATA_DIRS.lock().unwrap()
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
  log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();

  ctrlc::set_handler(move || {
    r.store(false, Ordering::SeqCst);
  }).expect("Error setting Ctrl-C handler");

  println!("Waiting for Ctrl-C...");

	let spawn = tokio::spawn(async move {
  // Amazon
	// let mut bots = run_amazon().await;

	// Newegg
		newegg_login().await;
		let mut bots = run_newegg().await;
		// cleanup(bots).await;
	});

  while running.load(Ordering::SeqCst) { }
  println!("\nShutting down joinem!");

  Ok(())
}

async fn action_handler(mut bot: &mut NeweggBot, action: Action) -> bool {
	match action {
		Stay => {
			delay_for(Duration::from_secs(2)).await;
		},
		Click(element) => {
			element.click().await;
			delay_for(Duration::from_secs(3)).await;
		},
		Wait => {
			let refresh_seconds = JOINEM_CONFIG.refresh_seconds();
			delay_for(Duration::from_secs(refresh_seconds)).await;
			bot.refresh().await;
		},
		End => {
			info!("NEWEGGLOGGEDIN");
			delay_for(Duration::from_secs(2)).await;
			return true;
		}
	}
	return false;
}

async fn newegg_login() {
  {
    let out_dir = JOINEM_CONFIG.newegg_chrome_user_data_template();

    let mut client = new_client(out_dir).await.expect("Failed to create new client!");
    let mut bot = NeweggBot::new(client, None).await;

    bot.goto_login().await;

		loop {
			let elements = NeweggElements::new(&mut bot).await;
			let action = bot.auto_login(&elements).await;
			if action_handler(&mut bot, action).await {
				break;
			}
		};

		bot.close().await;
	};
}

async fn run_newegg() -> Vec<Bot2> {
  let items = JOINEM_CONFIG.newegg_items.clone();
  let mut spawns = vec![]; 
  for item in items.into_iter() {
    let the_item = item.clone();
    debug!("Starting {:?}", item.name);
    let spawn = tokio::spawn(async move {
      let out_dir = JOINEM_CONFIG.find_or_create_data_folder();
      let mut client = new_client(out_dir).await.expect("Failed to create new client!");
      let mut bot = NeweggBot::new(client, Some(item.clone())).await;
      bot.goto().await;

      loop {
				let elements = NeweggElements::new(&mut bot).await;
				let action = bot.auto_purchase(&elements, item.clone()).await;

				if action_handler(&mut bot, action).await {
					break;
				}
			}

      bot.close().await;
    });

    spawns.push(Bot2{item: the_item, handle: spawn});
  }
  spawns
}

async fn run_amazon() -> Vec<Bot2> {
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

      let out_dir = JOINEM_CONFIG.find_or_create_data_folder();
      let client = new_client(out_dir).await.expect("Failed to create new client!");

      check_amazon_item(client, item.clone()).await;
    });
    spawns.push(Bot2{item: the_item, handle: spawn});

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

struct Bot2 {
  item: Item,
  handle: tokio::task::JoinHandle<()>
}

async fn cleanup(bots: Vec<Bot2>) {
  for bot in bots {
    println!("Doin somethin");
    // let enter = bot.handle;
    // println!("{:?}", enter);
    // println!("Doin somethin for {}", bot.name);
  }
  // fs::remove_dir_all("/tmp/joinem").unwrap();
  // rm_rf::ensure_removed("/tmp/joinem").expect("couldn't delete");
}
