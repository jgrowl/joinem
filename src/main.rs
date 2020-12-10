#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
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

#[macro_use]
extern crate lazy_static;

// use num::bigint::{BigInt};

// extern crate currency;
// use currency::Currency;

type Item = (String, f32, String);

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

// use num_bigint::{ToBigInt, RandBigInt};
// use num::traits::ToPrimitive;
use bigdecimal::BigDecimal;
use std::str::FromStr;

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

use std::{io, fs};

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

// use std::rand::{self, Rng};
use rand::prelude::*;

extern crate fs_extra;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;

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

extern crate config;

use std::env;
use config::{Config, File, FileFormat, Environment};
use std::collections::HashMap;

struct JoinemConfig {
  settings: HashMap<String, String>
}

impl JoinemConfig {
  fn new() -> JoinemConfig { 

  let mut settings = config::Config::default();
  settings
    // Add in `./Settings.toml`
    .merge(config::File::with_name("Settings").required(false)).unwrap()
    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    .merge(config::Environment::with_prefix("JOINEM")).unwrap();

    // // Add 'Settings.$(RUST_ENV).toml`
    // let name = format!("Settings.{}", env::var("env").unwrap_or("development".into()));
    // config.merge(File::new(&name, FileFormat::Toml).required(false)).unwrap();

    let settings =  settings.try_into::<HashMap<String, String>>().unwrap();
    JoinemConfig { settings: settings }
  }

  fn password(&self) -> String {
    self.settings.get("password").unwrap().clone()
  }

  fn username(&self) -> String {
    self.settings.get("username").unwrap().clone()
  }

  fn chrome_user_data(&self) -> String {
    self.settings.get("chrome_user_data").unwrap().clone()
  }

  fn newegg_items() {

    // GIGABYTE AORUS GeForce RTX 3090 DirectX 12 GV-N3090AORUS M-24GD 24GB Video Card + GIGABYTE GP-P850GM 850W ATX 12V v2.31 80 PLUS GOLD Certified Active (>0.9 typical) PFC Power Supply
    // 1,799.98
    // https://www.newegg.com/Product/ComboDealDetails?ItemList=Combo.4190467
    //
    //
    // GIGABYTE AORUS GeForce RTX 3090 DirectX 12 GV-N3090AORUS X-24GD 24GB 384-Bit GDDR6X PCI Express 4.0 x16 SLI Support ATX Video Card + GIGABYTE GP-P850GM 850W ATX Power Supply
    // $1,899.98
    // https://www.newegg.com/Product/ComboDealDetails?ItemList=Combo.4190485
    //
    //
    // GIGABYTE AORUS GeForce RTX 3080 DirectX 12 GV-N3080AORUS X-10GD 10GB 320-Bit GDDR6X PCI Express 4.0 x16 ATX Video Card + GIGABYTE GP-P850GM 850W ATX Power Supply
    // $999.98
    // https://www.newegg.com/Product/ComboDealDetails?ItemList=Combo.4190483
    //
    // GIGABYTE AORUS GeForce RTX 3080 DirectX 12 GV-N3080AORUS M-10GD 10GB 320-Bit GDDR6X PCI Express 4.0 x16 ATX Video Card + GIGABYTE GP-P850GM 850W ATX Power Supply
    // $949.98
    // https://www.newegg.com/Product/ComboDealDetails?ItemList=Combo.4190361
    //
    // GIGABYTE GP-P850GM 850W ATX 12V v2.31 80 PLUS GOLD Certified Full Modular Active (>0.9 typical) PFC Power Supply + GIGABYTE AORUS GeForce RTX 3080 DirectX 12 GV-N3080AORUSX W-10GD 10GB 320-Bit GDDR6X PCI Express 4.0 x16 ATX Video Card
    // $1,099.98
    // https://www.newegg.com/Product/ComboDealDetails?ItemList=Combo.4207165
    //
    // AMD Ryzen 9 5900X 12-Core 3.7 GHz Socket AM4 105W 100-100000061WOF Desktop Processor
    // $549.99
    // https://www.newegg.com/amd-ryzen-9-5900x/p/N82E16819113664
    //
    //AMD Ryzen 9 5950X 16-Core 3.4 GHz Socket AM4 105W 100-100000059WOF Desktop Processor
    //$799.99
    // https://www.newegg.com/amd-ryzen-9-5950x/p/N82E16819113663

  }

  fn items(&self) -> Vec<Item> {

  // out of stock
  // let url = "https://www.amazon.com/dp/B07XPC9B55/ref=twister_B08LYZMK9C?_encoding=UTF8&psc=1";

  // coffee, it works!
  // let url = "https://www.amazon.com/gp/product/B078TN99F9";

  // subscribe and save test
  // let url = "https://www.amazon.com/gp/product/B003SGHSCG?pf_rd_r=PJCXN2B304AV890R0GP2";
  //


    let items: Vec<Item> = vec![
(
 String::from("AMD Ryzen 5950X"), 
 850f32, 
 String::from("https://www.amazon.com/AMD-Ryzen-5950X-32-Thread-Processor/dp/B0815Y8J9N")
)

,(
 String::from("AMD Ryzen 5900X"),
 600f32,
 String::from("https://www.amazon.com/AMD-Ryzen-5900X-24-Thread-Processor/dp/B08164VTWH")
),

(
  String::from("Gigabyte 3080 AORUS-M"),
  850f32,
  String::from("https://www.amazon.com/Gigabyte-GeForce-Graphics-GV-N3080AORUS-M-10GD/dp/B08KJ3VKLQ")
)
,(
  String::from("Gigabyte 3090 AORUS-X"),
  1800f32,
  String::from("https://www.amazon.com/GIGABYTE-GeForce-Graphics-GV-N3090AORUS-X-24GD/dp/B08KTWVHQP"))
,(
  String::from("Gigabyte 3090 AORUS-X"),
  1800f32,
  String::from("https://www.amazon.com/GIGABYTE-GeForce-Graphics-GV-N3090AORUS-M-24GD/dp/B08KTYZXR9"))
,(
  String::from("Gigabyte 3090 GAMING-OC"),
  1800f32,
  String::from("https://www.amazon.com/Gigabyte-Graphics-WINDFORCE-GV-N3090GAMING-OC-24GD/dp/B08HJRF2CN")
)

];
    items
  }
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

extern crate ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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


// #![deny(warnings)]
// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
//
// mod routes;
//
// use std::error::Error;
// use std::convert::Infallible;
//
// use futures::future::TryFutureExt;
//
// use hyper::server::Server;
//
// use listenfd::ListenFd;
//
// use tokio;
// use tokio::net::TcpListener;
// use tokio::sync::oneshot;
// use tokio::sync::oneshot::{Sender, Receiver};
//
//
// use warp::Filter;
// extern crate futures;
// extern crate warp;
//
// extern crate listenfd;
//
// // fn main() -> Result<(), Box<dyn Error>> {
// //  let mut runtime = tokio::runtime::Builder::new().core_threads(1).enable_io().build()?;
// //
// //
// //
// //     runtime.block_on(async {
//
// #[tokio::main]
// async fn main() {
//   println!("Started...");
//
//   pretty_env_logger::init();
//
//   let routes = routes::get_routes();
//
//
//   let mut listenfd = ListenFd::from_env();
//   let (tx, rx) = oneshot::channel();
//     // if listenfd doesn't take a TcpListener (i.e. we're not running via
//     // the command above), we fall back to explicitly binding to a given
//     // host:port.
//     // let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
//     //
//   if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
//
//       // hyper let's us build a server from a TcpListener (which will be
//       // useful shortly). Thus, we'll need to convert our `warp::Filter` into
//       // a `hyper::service::MakeService` for use with a `hyper::server::Server`.
//       let svc = warp::service(routes);
//
//       let make_svc = hyper::service::make_service_fn(|_: _| {
//         // the clone is there because not all warp filters impl Copy
//         let svc = svc.clone();
//         async move { Ok::<_, Infallible>(svc) }
//       });
//
//       // let server = Server::from_tcp(l).unwrap();
//       let server = Server::from_tcp(l).unwrap();
//       server.serve(make_svc).await.unwrap();
//     } else {
//
//       let (addr, server) = warp::serve(routes)
//         // let (addr, server) = warp::serve(make_svc)
//         .bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
//           rx.await.ok();
//         });
//
//       // Spawn the server into a runtime
//       tokio::task::spawn(server);
//
//       // Later, start the shutdown...
//       let _ = tx.send(());
//
//
//       // let server = Server::bind(&([127, 0, 0, 1], 3030).into());
//       // server.serve(make_svc).await.unwrap();
//     };
//
//     //
//     // });
//     //
//     // Ok(())
// }
