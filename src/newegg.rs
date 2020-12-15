use std::time::{Duration};
use log::{info, warn, debug};
use std::str::FromStr;
use tokio::time::delay_for;
use fantoccini::{Client, Locator};
use async_std::future;
use std::process;

use crate::JOINEM_CONFIG;
use crate::config::Item;

pub async fn is_logged_in(mut client: & mut Client) -> bool {
  let mut myaccount = client.find(Locator::Id("myaccount")).await;
  let is_ok = myaccount.is_ok();

  // println!("{:?}", is_ok);
  is_ok
}

  // delay_for(Duration::from_secs(4)).await;
//
  // if (!is_logged_in(c2).await) {
    // newegg_login(c2).await;
  // }
  //
pub async fn newegg_buy(mut client: & mut Client) -> Result<(), fantoccini::error::CmdError> {
  // let mut e = client.find(Locator::Css("a.atnPrimary")).await.unwrap();
  let mut e = client.find(Locator::Css("button.btn-primary")).await.unwrap();
  e.click().await;
  Ok(())
}

pub async fn reject_coverage(mut client: & mut Client) -> Result<(), fantoccini::error::CmdError> {
  // let mut e = client.find(Locator::Id("modal-intermediary")).await.unwrap();
  // let mut dismiss = client.find(Locator::Css("button[data-dismiss='modal']")).await.unwrap();
  let mut dismiss = client.find(Locator::Css("button[data-dismiss='modal']")).await;
	match dismiss {
		Ok(ok) => { ok.click().await; },
		Err(err) => { // No popup to click past
		}
	};

  Ok(())
}

pub async fn newegg_confirm(mut client: & mut Client) -> Result<(), fantoccini::error::CmdError> {
// div[role='document'] button.btn-primary
  let mut e = client.find(Locator::Css("div[role='document'] button.btn-primary")).await.unwrap();
	e.click().await;
  Ok(())
}

pub async fn newegg_purchase(mut client: & mut Client) -> Result<(), fantoccini::error::CmdError> {
  let mut e = client.find(Locator::Css("button.btn-primary")).await.unwrap();
	e.click().await;
  Ok(())
}

pub fn udata_string(utag_data: &serde_json::Value, name: &str) -> Option<String> {
	let element: String = utag_data[name].as_str().unwrap_or_else(||{""}).to_owned();
	if element.eq("") {
		None
	} else {
		Some(element)
	}
}

pub fn udata_array(utag_data: &serde_json::Value, name: &str) -> Option<String> {
	let element = utag_data[name].as_array().unwrap()[0].as_str().unwrap_or_else(|| {""});
	let element = if element.eq("") {
		None
	} else {
		// Some(product_sale_price.parse::<f32>())
		Some(element.to_owned())
	};
	element
}

pub fn udata_array_f32(utag_data: &serde_json::Value, name: &str) -> Option<f32> {
	let element = utag_data[name].as_array().unwrap()[0].as_str().unwrap_or_else(|| {""});
	let element = if element.eq("") {
		None
	} else {
		Some(element.parse::<f32>().unwrap())
	};
	element
}

pub fn udata_array_bool(utag_data: &serde_json::Value, name: &str) -> Option<bool> {
	let element = utag_data[name].as_array().unwrap()[0].as_str().unwrap_or_else(|| {""});
	let element = if element.eq("") {
		None
	} else {
		Some(element.eq("1"))
	};
	element
}

//window.utag_data
pub async fn newegg_utag_data(mut client: & mut Client) -> Result<(), fantoccini::error::CmdError> {
	let utag_data = client.execute("return utag_data", vec![]).await.unwrap();

	let user_name: Option<String> =  udata_string(&utag_data, "user_name");
	let page_name: Option<String> = udata_string(&utag_data, "page_name");

	let product_sale_price = udata_array_f32(&utag_data, "product_sale_price");
	let product_instock = udata_array_bool(&utag_data,"product_instock");

// product_instock

	// let product_sale_price: Array<String> = utag_data["product_sale_price"].as_array().unwrap_or_else(||{[]}).to_owned();
	// let product_sale_price = product_sale_price[0];
  //
	// let product_sale_price: Option<f64> = if product_sale_price.eq("") {
	//   None
	// } else {
	//   Some(product_sale_price.parse().unwrap())
	// };

// "NewProductDetail" "ShoppingCart"
//"cart_grand_total": "29.99"
// site_state: 0





	println!("YOYO: {:?}, {:?}, {:?}, {:?}", user_name, page_name, product_sale_price, product_instock);
  // let mut e = client.find(Locator::Css("button.btn-primary")).await.unwrap();
	// e.click().await;
	Ok(())
}


pub async fn check_newegg_item(mut c2: & mut Client, url: Item) -> Result<bool, fantoccini::error::CmdError> {
  c2.goto(&url.url.clone()).await?;

    if is_available(c2).await {
      info!("AVAILABLE\t{}", url.name);

      let price = scrape_price(& mut c2).await;
      if price <= url.max_price {

        info!("AFFORDABLE\t{}", url.name);

        return Ok(true);

      } else {
        debug!("EXPENSIVE\t{}", url.name);
      }
    } else {
      debug!("NOSTOCK\t{}", url.name);
    }

    Ok(false)
}


async fn is_available(c: & mut Client)-> bool {
  // modal-content


	// For buys: "btn-primary"
	// For notifiies "atnPrimary"
  // document.getElementsByClassName('atnPrimary')[0].innerText
  // let mut e = c.wait_for_find(Locator::Css(".atnPrimary")).await.unwrap();
  let mut e = c.wait_for_find(Locator::Css(".btn-primary")).await.unwrap();
  // let e = c.find(Locator::Id("atnPrimary")).await.unwrap();
  // println!("{:?}", e.text().await.unwrap());
  let text = e.text().await.unwrap();
  // if text.eq("AUTO NOTIFY") {
  //   return false;
  // }

  let normalized = text.to_uppercase().replace(" ", "");
  if normalized.eq("ADDTOCART") {
    return true;
  }

  false
}

async fn scrape_price(c: & mut Client)-> f32 {
	let dollar_string = match c.find(Locator::Css(".price-current strong")).await {
		Ok(mut ok) => { ok.text().await.unwrap() },
		Err(e) => {
			let e = c.find(Locator::Id("singleFinalPrice")).await;
			let price = e.unwrap().attr("content").await.unwrap().unwrap();
			price
		}
	};

  let dollar_string = dollar_string.replace("$", "");
  let dollar_string = dollar_string.replace(",", "");

  let float = f32::from_str(&dollar_string).unwrap();

  float
}


pub async fn newegg_login_at_checkout(c: & mut Client)  -> Result<(), fantoccini::error::CmdError>{

// https://secure.newegg.com/NewMyAccount/AccountLogin.aspx
// labeled-input-signEmail
// signInSubmit

  let mut email_input = c.find(Locator::Id("labeled-input-signEmail")).await?;
  let username = JOINEM_CONFIG.newegg_username.to_owned();
  email_input.send_keys(&username).await?;
  let signin_button = c.find(Locator::Id("signInSubmit")).await?;
  signin_button.click().await?;

  delay_for(Duration::from_secs(3)).await;

  let mut password_input = c.find(Locator::Id("labeled-input-password")).await?;
  let password = JOINEM_CONFIG.newegg_password.to_owned();
  password_input.send_keys(&password).await?;

  let signin_button= c.find(Locator::Id("signInSubmit")).await?;
  signin_button.click().await?;

  delay_for(Duration::from_secs(3)).await;

  Ok(())
}


pub async fn newegg_login(c: & mut Client)  -> Result<(), fantoccini::error::CmdError>{

// https://secure.newegg.com/NewMyAccount/AccountLogin.aspx
// labeled-input-signEmail
// signInSubmit


  // // let amazon_sign_in_url = "https://www.amazon.com/sign/s?k=sign+in";
  let newegg_sign_in_url = "https://secure.newegg.com/NewMyAccount/AccountLogin.aspx";
  //
  c.goto(newegg_sign_in_url).await?;
  // // let url = c.current_url().await?;
  // //
  let mut email_input = c.find(Locator::Id("labeled-input-signEmail")).await?;
  let username = JOINEM_CONFIG.newegg_username.to_owned();
  email_input.send_keys(&username).await?;
  let signin_button = c.find(Locator::Id("signInSubmit")).await?;
  signin_button.click().await?;

  delay_for(Duration::from_secs(3)).await;

  let mut password_input = c.find(Locator::Id("labeled-input-password")).await?;
  let password = JOINEM_CONFIG.newegg_password.to_owned();
  password_input.send_keys(&password).await?;

  let signin_button= c.find(Locator::Id("signInSubmit")).await?;
  signin_button.click().await?;

  delay_for(Duration::from_secs(3)).await;

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
