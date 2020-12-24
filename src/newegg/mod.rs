pub mod utag_data;
pub mod elements;

use crate::newegg::elements::NeweggElements;
use std::convert::TryInto;
use std::time::{Duration};
use std::str::FromStr;
use std::process;
use log::{info, warn, debug};
use tokio::time::delay_for;
use async_std::future;
use fantoccini::{Client, Locator, Element};

use crate::config::Item;
use crate::types::{ElementResult, Action::{Click, Wait, End, Stay}};
use crate::JOINEM_CONFIG;

use utag_data::Utag_Data;
use crate::types::Action;

pub struct Bot {
  pub client: Client,
  pub item: Option<Item>
}

impl Bot {

  pub async fn new(client: Client, item: Option<Item>) -> Bot {
    let mut bot = Bot{client, item};
    // bot.goto().await;

    bot
  }

  pub async fn goto(&mut self) {
      self.client.goto(&self.item().url.clone()).await;
  }

  pub async fn goto_login(&mut self) {
    let url = JOINEM_CONFIG.newegg_sign_in_url.clone();
      self.client.goto(&url).await;
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

    debug!("CARDFOUND\t{}", self.item().name);
    let card_number = JOINEM_CONFIG.card_number.to_owned().unwrap();
    card_number_input.clone().click().await;
    // card_number_input.clear().await;

    match card_number_input.send_keys(&card_number).await {
      Ok(success) => { 
        debug!("CARDFILL\t{}", self.item().name); 
      },
      Err(err) => {
        debug!("CARDFILLFAILED\t{}", self.item().name)
      }
    };

    Some(card_number_input)
  }

  pub async fn get_cvv_el_and_try_fill(&mut self) -> Option<Element> {
    let cvv_selector = JOINEM_CONFIG.cvv_selector.to_owned().unwrap();
    match self.client.find(Locator::Css(&cvv_selector)).await {
      Ok(mut cvv_input) => {
        debug!("CVV4FOUND\t{}", self.item().name);
        let cvv = JOINEM_CONFIG.cvv.to_owned().unwrap();
        cvv_input.clone().click().await;
        cvv_input.clear().await;
        match cvv_input.send_keys(&cvv).await {
          Ok(success) => { debug!("CVV4FILL\t{}", self.item().name); },
          Err(err) => {
            debug!("CVV4FILLFAILED\t{}", self.item().name)
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

    debug!("NEWEGGEMAILFOUND");
    let username = JOINEM_CONFIG.newegg_username.to_owned();
    match username_input.send_keys(&username).await {
          Ok(success) => { debug!("NEWEGGEMAILFILL") },
          Err(err) => {
            debug!("EMAILFILLFAILED")
          }
    }

    Some(username_input)
  }

  pub async fn get_password_el_and_try_fill(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.password_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Id(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    debug!("PASSFOUND");
    let value = JOINEM_CONFIG.newegg_password.to_owned();

    match element.send_keys(&value).await {
          Ok(success) => { debug!("PASSFILLSUCCESS"); },
          Err(err) => {
            debug!("PASSFILLFAILED")
          }
    }

    Some(element)
  }


    // let survey = self.client.find(Locator::Css("a.centerPopup-trigger-close")).await;
    // let sign_in_submit = self.client.find(Locator::Id("signInSubmit")).await;

  pub async fn get_sign_in_submit_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.sign_in_submit_selector.to_owned().unwrap();
		let element = self.get_el(selector).await;
		if element.is_some() {
			debug!("SIGNINSUBMITFOUND");
		}

    element
  }


  pub async fn get_ec_frame_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.ec_frame_selector.to_owned().unwrap();
		let element = self.get_el(selector).await;
		if element.is_some() {
			debug!("ECFRAMEFOUND\t{}", self.item().name);
		}

    element
  }


  pub async fn get_survey_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.survey_selector.to_owned().unwrap();
		let element = self.get_el(selector).await;
		if element.is_some() {
			debug!("SURVEYFOUND\t{}", self.item().name);
		}

    element
  }


  pub async fn get_add_to_cart_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.add_to_cart_selector.to_owned().unwrap();
		let element = self.get_el(selector).await;
			if element.is_some() {
				debug!("ADDTOCART\t{}", self.item().name);
			}

		element
  }

  pub async fn get_insurance_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.insurance_selector.to_owned().unwrap();
		let element = self.get_el_with_text(selector, "NO,THANKS".to_owned()).await;
			if element.is_some() {
				debug!("INSURANCEFOUND\t{}", self.item().name);
			}

		element
  }

  pub async fn get_success_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.success_selector.to_owned().unwrap();
		self.get_el(selector).await
  }

  pub async fn get_el(&mut self, selector: String) -> Option<Element> {
    let mut element = self.client.find(Locator::Css(&selector)).await;
		match element {
			Ok(element) => Some(element),
			Err(e) => None
		}
  }

  pub async fn get_el_with_text(&mut self, selector: String, content: String) -> Option<Element> {
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    let text = element.text().await;
    let mut r = None;
    if text.is_ok() {
      let text = text.unwrap().to_uppercase().replace(" ", "");
      if text.eq(&content) {
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
        debug!("PROMOTIONFOUND\t{}", self.item().name);
        r = Some(element);
      } 
    } 

    r
  }

  pub async fn get_sign_in_el(&mut self) -> Option<Element> {
    let selector = JOINEM_CONFIG.sign_in_selector.to_owned().unwrap();
    let mut element = self.client.find(Locator::Css(&selector)).await;
    if element.is_err() { return None; };
    let mut element = element.unwrap();

    let text = element.text().await;
    let mut r = None;
    if text.is_ok() {
      let text = text.unwrap().to_uppercase().replace(" ", "");
      if text.eq("SIGNIN/REGISTER") {
        debug!("SAVEFOUND\t{}", self.item().name);
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
        debug!("SAVEFOUND\t{}", self.item().name);
        r = Some(element);
      } 
    } 

    r
  }

  pub fn item(&self) -> Item {
    self.item.clone().unwrap()
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


	pub async fn auto_purchase(&mut self, elements: &NeweggElements, item: Item) -> Action {
    let current_url = self.client.current_url().await.unwrap();
    let path = current_url.path();

    // Check if this is neccessary. Put it here because element wasns't
    // showing up unless scrolled down, but it might have been a problem
    // with find vs findall where I was only getting the first element
    self.scroll_to_bottom().await;
    // self.resize().await;

    if elements.survey_el.is_some() {
      // debug!("SKIPSURVEY\t{}", item.name);
      // The survey is annoying because even if you dismiss it, it will 
      // stick around in the background hidden. So just handle it here
      // and remove the node when finishe

      let survey = elements.survey_el.as_ref().unwrap().clone();
      survey.click().await;
      self.client.execute("document.querySelector('a.centerPopup-trigger-close').remove()", vec![]).await.unwrap();
			return Stay 
    } else if elements.insurance_el.is_some() {
      debug!("SKIPINCOVERAGE\t{}", item.name);
      Click(elements.insurance_el.as_ref().unwrap().clone())
    } else if elements.promotion_el.is_some() {
      debug!("SKIPPROMO\t{}", item.name);
      Click(elements.promotion_el.as_ref().unwrap().clone())
    }  else if elements.sign_in_submit_el.is_some() {
      debug!("SIGNIN\t{}", item.name);
      Click(elements.sign_in_submit_el.as_ref().unwrap().clone())
    } else if elements.save_el.is_some(){
      debug!("SAVE\t{}", item.name);
      self.try_save().await;
			return Stay;
    } else if elements.continue_to_payment_el.is_some() {
      debug!("CONTINUETOPAYMENT\t{}", item.name);
      Click(elements.continue_to_payment_el.as_ref().unwrap().clone())
    } else if elements.view_cart_el.is_some() {
      debug!("VIEWCART\t{}", item.name);
      Click(elements.view_cart_el.as_ref().unwrap().clone())
    } else if elements.add_to_cart_el.is_some() {
      debug!("FOUNDADDTOCART\t{}", item.name);

      if elements.utag_data.is_none() 
				|| elements.utag_data.as_ref().unwrap().product_instock.is_none() 
        || elements.utag_data.as_ref().unwrap().product_sale_price.is_none() {
          debug!("NOUDATA\t{}", item.name);
          return Wait;
      }

      let instock = elements.utag_data.as_ref().unwrap().product_instock.as_ref().unwrap().clone();
      let sale_price = elements.utag_data.as_ref().unwrap().product_sale_price.unwrap();
      if !instock {
        debug!("NOSTOCK\t{}", item.name);
        return Wait;
      }

      if sale_price > item.max_price {
        debug!("EXPENSIVE\t{}", item.name);
        return Wait;
      }

      Click(elements.add_to_cart_el.as_ref().unwrap().clone())
    } else if elements.secure_checkout_el.is_some(){
      debug!("SECURECHECKOUT\t{}", item.name);
      Click(elements.secure_checkout_el.as_ref().unwrap().clone())
    } else {
      debug!("SLEEP\t{}", item.name);
      return Wait;
    }
      // TODO: CHECK SAVE
      // Review your order
  }

	pub async fn auto_login(&mut self, elements: &NeweggElements) -> Action {
		if let Some(utag_data) = &elements.utag_data {
			if let Some(user_name) = &utag_data.user_name {
				return End
			}
		}
		
		if elements.sign_in_submit_el.is_some() {
			debug!("NEWEGGSIGNIN");
			Click(elements.sign_in_submit_el.clone().unwrap())
		} else if elements.sign_in_el.is_some() {
			Click(elements.sign_in_el.clone().unwrap())
		} else {
			debug!("SLEEP");
			Wait
		}
	}
	}

      // self.client.clone().enter_parent_frame().await;


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
