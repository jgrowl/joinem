use std::time::{Duration};
use log::{info, warn, debug};
use std::str::FromStr;
use tokio::time::delay_for;
use fantoccini::{Client, Locator};
use async_std::future;
use std::process;

use crate::JOINEM_CONFIG;
use crate::config::Item;

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
  let username = JOINEM_CONFIG.username.to_owned();
  search_input.send_keys(&username).await?;
  search_form.submit().await?;
  // WARNING: Look at this. Shouldn't I wait to submit after sending 
  // password too?
  // 

  let search_form = c.form(Locator::Css("form[name='signIn']")).await?;
  let mut search_input = c.find(Locator::Id("ap_password")).await?;
  let password = JOINEM_CONFIG.password.to_owned();
  search_input.send_keys(&password).await?;
  search_form.submit().await?;
  ////

  let e = c.wait_for_find(Locator::Id("nav-link-accountList")).await?;
  println!("Logged in!");
  Ok(())
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

  match future::timeout(Duration::from_secs(5), buy_now).await {
    Ok(button) => {
      // Need to wait some time or else will take to shopping cart
      delay_for(Duration::from_secs(5)).await;

      button.unwrap().click().await;
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

  // let form = c.form(Locator::Id("place-order-form")).await;
  // form.unwrap().submit().await;

  // // let place_order_id = "turbo-cel-place-order-button";
  // let place_order_id = "turbo-checkout-pyo-button";
  let place_order_id = "turbo-checkout-place-order-button";
  let place_order = c.wait_for_find(Locator::Id(place_order_id)).await;
  place_order.unwrap().click().await;

  info!("You got it dude!");

  Ok(())
}

pub async fn check_amazon_item(mut c2: Client, url: Item) -> Result<(), fantoccini::error::CmdError> {
  // let mut c2 = new_client().await.expect("Failed to create new client!");
  c2.goto(&url.url).await?;

  loop {
    if is_buy_now_amazon(& mut c2).await {
      let price = scrape_price(& mut c2).await;
      info!("AVAILABLE\t{}", url.name);
      if price <= url.max_price {
        info!("AFFORDABLE\t{}", url.name);


        buy_now(& mut c2).await;

        // Reject coverage if offered!
        info!("{} Rejecting coverage!", url.name);
        reject_coverage(& mut c2).await;


        delay_for(Duration::from_secs(4)).await;

        // Login if asked
        let current_url = c2.current_url().await?;
        let path = current_url.path();

        // "https://www.amazon.com/ap/signin"
        info!("{} Logging in!", url.name);
        let sign_in_path = "/ap/signin";
        if path.eq(sign_in_path) {
          let search_form = c2.form(Locator::Css("form[name='signIn']")).await?;
          let mut search_input = c2.find(Locator::Id("ap_password")).await?;
          let password = JOINEM_CONFIG.password.to_owned();
          search_input.send_keys(&password).await?;
          search_form.submit().await?;
        }


        delay_for(Duration::from_secs(3)).await;
        let current_url = c2.current_url().await?;
        let path = current_url.path();

        ////
        //https://www.amazon.com/gp/buy/spc/handlers/display.html?hasWorkingJavascript=1
        // let cart_path = "/ap/signin";
        //
        let cart_path = "/gp/buy/spc/handlers/display.html";
        if path.eq(cart_path) {
          debug!("It took us to shopping cart!");
          let mut submit_order = c2.find(Locator::Id("submitOrderButtonId")).await?;
          submit_order.click().await;
        } else { // We're still on product page with pop up
          // confirm
          confirm_buy_now(& mut c2).await;
        }

        debug!("BUYCONFIRMED\t{}", url.name);

        delay_for(Duration::from_secs(15)).await;
        // TODO: For now just exit if one is successful
        process::exit(0x0100);
        break;
      } else {
        debug!("EXPENSIVE\t{}", url.name);
      }
    } else { 
      debug!("NOSTOCK\t{}", url.name);
    }

    // info!("{}, cannot be bought now! Sleeping...", url.name);
    delay_for(Duration::from_secs(15)).await;
    c2.refresh().await;
  }

  c2.close().await;
  Ok(())

}

// use url::{Url, ParseError};
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    // let url = url::Url::parse("https://www.amazon.com/gp/buy/spc/handlers/display.html?hasWorkingJavascript=1").expect("yo");
    // let path = url.path();
    // // let path = current_url.path();
    //
    //   assert_eq!(path, "hol");
  }
}
