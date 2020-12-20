mod utag_data;

use std::convert::TryInto;
use std::time::{Duration};
use std::str::FromStr;
use std::process;
use log::{info, warn, debug};
use tokio::time::delay_for;
use async_std::future;
use fantoccini::{Client, Locator, Element};

use crate::config::Item;
use crate::types::ElementResult;
use crate::JOINEM_CONFIG;


// pub struct Bot<'a> {
pub struct Bot {
  // pub client: &'a mut Client,
  pub client: Client,
  pub item: Item
}


// impl <'a> Bot<'a> {
impl Bot {

  pub async fn new(client: Client, item: Item) -> Bot {
    let mut bot = Bot{client, item};
    bot.goto().await;

    bot
  }

  pub async fn goto(&mut self) {
      self.client.goto(&self.item.url.clone()).await;
  }

  pub async fn close(&mut self) {
      self.client.close().await;
  }

  pub async fn refresh(&mut self) {
      self.client.refresh().await;
  }

  pub async fn get_card_number_el_and_try_fill(&mut self) -> Option<Element> {
    // <input type="text" class="form-text is-wide mask-cardnumber" aria-label="Card Number" value="">
    let card_number_selector = JOINEM_CONFIG.card_number_selector.to_owned().unwrap();
    let element = self.client.find(Locator::Css(&card_number_selector)).await;
    if element.is_err() { return None; };

    let mut card_number_input = element.unwrap();

    debug!("CARDFOUND\t{}", self.item.name);
    let card_number = JOINEM_CONFIG.card_number.to_owned().unwrap();
    card_number_input.clone().click().await;
    // card_number_input.clear().await;

    match card_number_input.send_keys(&card_number).await {
      Ok(success) => { 
        debug!("CARDFILL\t{}", self.item.name); 
      },
      Err(err) => {
        debug!("CARDFILLFAILED\t{}", self.item.name)
      }
    };

    Some(card_number_input)
  }

  pub async fn get_cvv_el_and_try_fill(&mut self) -> Option<Element> {
    let cvv_selector = JOINEM_CONFIG.cvv_selector.to_owned().unwrap();
    match self.client.find(Locator::Css(&cvv_selector)).await {
      Ok(mut cvv_input) => {
        debug!("CVV4FOUND\t{}", self.item.name);
        let cvv = JOINEM_CONFIG.cvv.to_owned().unwrap();
        cvv_input.clone().click().await;
        cvv_input.clear().await;
        match cvv_input.send_keys(&cvv).await {
          Ok(success) => { debug!("CVV4FILL\t{}", self.item.name); },
          Err(err) => {
            debug!("CVV4FILLFAILED\t{}", self.item.name)
          }
        };
        Some(cvv_input)
      },
      Err(e)=> {None}
    }
  }

  pub async fn get_username_el_and_try_fill(&mut self) -> Option<Element> {
    let username_selector = JOINEM_CONFIG.username_selector.to_owned().unwrap();
    let element = self.client.find(Locator::Id(&username_selector)).await;
    if element.is_err() { return None; };
    let mut username_input = element.unwrap();

    debug!("EMAILFOUND\t{}", self.item.name);
    let username = JOINEM_CONFIG.newegg_username.to_owned();
    match username_input.send_keys(&username).await {
          Ok(success) => { debug!("EMAILFILL\t{}", self.item.name); },
          Err(err) => {
            debug!("EMAILFILLFAILED\t{}", self.item.name)
          }
    }

    Some(username_input)
  }

  pub async fn get_password_el_and_try_fill(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.password_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Id(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    debug!("PASSFOUND\t{}", self.item.name);
    let value = JOINEM_CONFIG.newegg_password.to_owned();

    match element.send_keys(&value).await {
          Ok(success) => { debug!("PASSFILLSUCCESS\t{}", self.item.name); },
          Err(err) => {
            debug!("PASSFILLFAILED\t{}", self.item.name)
          }
    }

    Some(element)
  }


    // let survey = self.client.find(Locator::Css("a.centerPopup-trigger-close")).await;
    // let sign_in_submit = self.client.find(Locator::Id("signInSubmit")).await;

  pub async fn get_sign_in_submit_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.sign_in_submit_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    debug!("SIGNINSUBMITFOUND\t{}", self.item.name);

    Some(element)
  }


  pub async fn get_ec_frame_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.ec_frame_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    debug!("ECFRAMEFOUND\t{}", self.item.name);

    Some(element)
  }


  pub async fn get_survey_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.survey_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    debug!("SURVEYFOUND\t{}", self.item.name);

    Some(element)
  }


  pub async fn get_add_to_cart_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.add_to_cart_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    debug!("ADDTOCART\t{}", self.item.name);

    Some(element)
      // TODO: Check that text actually is add to cart
  }


  pub async fn get_insurance_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.insurance_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    let text = element.text().await;
    let mut r = None;
    if text.is_ok() {
      let text = text.unwrap().to_uppercase().replace(" ", "");
      if text.eq("NO,THANKS") {
        debug!("INSURANCEFOUND\t{}", self.item.name);
        r = Some(element);
      } 
    } 

    r
  }

  pub async fn get_continue_to_payment_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.continue_to_payment_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();
    Some(element)
  }

  pub async fn get_promotion_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.promotion_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    let text = element.text().await;
    let mut r = None;
    if text.is_ok() {
      let text = text.unwrap().to_uppercase().replace(" ", "");
      if text.eq("I'MNOTINTERESTED.") {
        debug!("PROMOTIONFOUND\t{}", self.item.name);
        r = Some(element);
      } 
    } 

    r
  }

  pub async fn get_save_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.save_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    let text = element.text().await;
    let mut r = None;
    if text.is_ok() {
      let text = text.unwrap().to_uppercase().replace(" ", "");
      if text.eq("SAVE") {
        debug!("SAVEFOUND\t{}", self.item.name);
        r = Some(element);
      } 
    } 

    r
  }


  pub async fn get_view_cart_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.view_cart_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();
    Some(element)
  }


  pub async fn get_secure_checkout_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.secure_checkout_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();
    Some(element)
      // TODO: check inner text?
  }

  pub async fn scroll_to_bottom(&mut self)  {
    self.client.execute("window.scrollBy(0,99999)", vec![]).await.unwrap();
  }

  pub async fn try_save(&mut self)  {

    // document.querySelectorAll('iframe').forEach( item =>
    // console.log(item.contentWindow.document.body.querySelectorAll('a'))
    // )

    let js = "document.querySelector(\"iframe[title='ec_payment']\").contentWindow.document.querySelector('button.btn-primary').click()";

    self.client.execute(js, vec![]).await.unwrap();
  }

  // pub async fn resize(&mut self) {
  //   let (width, height) = self.client.get_window_size().await.unwrap();
  //   let width: u32 = width.try_into().unwrap();
  //   let height: u32 = height.try_into().unwrap();
  //   self.client.set_window_size(width+50u32, height+50u32).await;
  //   self.client.set_window_size(width, height).await;
  // }

  pub async fn auto(&mut self, item: Item) -> ElementResult {
    let current_url = self.client.current_url().await?;
    let path = current_url.path();

    // Check if this is neccessary. Put it here because element wasns't
    // showing up unless scrolled down, but it might have been a problem
    // with find vs findall where I was only getting the first element
    self.scroll_to_bottom().await;
    // self.resize().await;

    let utag_data = self::utag_data::newegg_utag_data(&mut self.client).await?;

    let cvv_el = self.get_cvv_el_and_try_fill().await;
    let username_el = self.get_username_el_and_try_fill().await;
    let password_el = self.get_password_el_and_try_fill().await;

    let sign_in_submit_el = self.get_sign_in_submit_el().await;
    let survey_el = self.get_survey_el().await;
    let insurance_el = self.get_insurance_el().await;
    let promotion_el = self.get_promotion_el().await;
    let continue_to_payment_el = self.get_continue_to_payment_el().await;
    let view_cart_el = self.get_view_cart_el().await;
    let add_to_cart_el = self.get_add_to_cart_el().await;
    let secure_checkout_el = self.get_secure_checkout_el().await;

    let mut card_number_el = None;
    let mut save_el = None;
    let mut ec_frame_el = self.get_ec_frame_el().await;
    if ec_frame_el.is_some() {
      let frame = ec_frame_el.unwrap();

      frame.enter_frame().await;
      save_el = self.get_save_el().await;

      card_number_el = self.get_card_number_el_and_try_fill().await;
      self.client.clone().enter_parent_frame().await;
    }



    // ADDTOCART, VIEWCART: https://www.newegg.com/p/0GA-0105-00040?Item=9SIACD5CS57992&quicklink=true
    //
    // CONTINUETOPAYMENT: https://secure.newegg.com/shop/checkout?sessionId=XOWN2QAEUGLBRJS33211F
    //
    // SECURECHECKOUT: https://secure.newegg.com/shop/cart
    // https://secure.newegg.com/shop/checkout


    // https://secure.newegg.com/shop/cart
    // SECURECHECKOUT 
    // https://secure.newegg.com/shop/checkout?sessionId=ETQBLO8QYUO7OSS702CC
    // "CONTINUETOPAYMENT"

    let mut clickable = if survey_el.is_some() {
      // debug!("SKIPSURVEY\t{}", item.name);
      // The survey is annoying because even if you dismiss it, it will 
      // stick around in the background hidden. So just handle it here
      // and remove the node when finishe

      let survey = survey_el.unwrap();
      survey.click().await;
      self.client.execute("document.querySelector('a.centerPopup-trigger-close').remove()", vec![]).await.unwrap();

      let placeholder = self.client.find(Locator::Css("p")).await.unwrap();
      return Ok(Some(placeholder));

    } else if insurance_el.is_some() {
      debug!("SKIPINCOVERAGE\t{}", item.name);
      insurance_el.unwrap()
    } else if promotion_el.is_some() {
      debug!("SKIPPROMO\t{}", item.name);
      promotion_el.unwrap()
    }  else if sign_in_submit_el.is_some() {
      debug!("SIGNIN\t{}", item.name);
      sign_in_submit_el.unwrap()
    } else if save_el.is_some(){
      debug!("SAVE\t{}", item.name);

      self.try_save().await;
      debug!("YO BUY THIS THANG");

      //Place Order button needs to be handled!

      let placeholder = self.client.find(Locator::Css("p")).await.unwrap();
      return Ok(Some(placeholder));

    //   document.querySelectorAll('iframe').forEach( item =>
    // console.log(item.contentWindow.document.body.querySelectorAll('a'))
    //   )


      // save_el.unwrap()

    } else if continue_to_payment_el.is_some() {
      debug!("CONTINUETOPAYMENT\t{}", item.name);
      continue_to_payment_el.unwrap()
    } else if view_cart_el.is_some() {
      debug!("VIEWCART\t{}", item.name);
      view_cart_el.unwrap()
    } else if add_to_cart_el.is_some() {
      debug!("FOUNDADDTOCART\t{}", item.name);

      if utag_data.product_instock.is_none() 
        || utag_data.product_sale_price.is_none() {
          debug!("NOUDATA\t{}", item.name);
          return Ok(None);
      }

      let instock = utag_data.product_instock.unwrap();
      let sale_price = utag_data.product_sale_price.unwrap();
      if !instock {
        debug!("NOSTOCK\t{}", item.name);
        return Ok(None);
      }

      if sale_price > item.max_price {
        debug!("EXPENSIVE\t{}", item.name);
        return Ok(None);
      }
      add_to_cart_el.unwrap()
    } else if secure_checkout_el.is_some(){
      debug!("SECURECHECKOUT\t{}", item.name);
      secure_checkout_el.unwrap()
    } else {
      warn!("NOCLICKABLES\t{}", item.name);
      return Ok(None);
    };

    Ok(Some(clickable))

      // TODO: CHECK SAVE
      // Review your order
  }
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
