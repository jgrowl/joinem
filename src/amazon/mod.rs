pub mod elements;

use log::{info, warn, debug};
use fantoccini::{Client, Locator, Element, Form};
use crate::JOINEM_CONFIG;
use crate::bot::Bot as BaseBot;
use crate::Item;

use crate::types::Action;
use crate::types::{ElementResult, Action::*};
use crate::amazon::elements::AmazonElements;

pub struct Bot {
  pub client: Client,
  pub item: Option<Item>
}

impl Bot {
  pub async fn new(client: Client, item: Option<Item>) -> Bot {
    let mut bot = Bot{client, item};
    bot
  }

  pub async fn goto_login(&mut self) {
    let url = JOINEM_CONFIG.amazon.sign_in_url.clone();
      self.client.goto(&url).await;
  }

  pub async fn close(&mut self) {
      self.client.close().await;
  }

	async fn get_reject_coverage_el(&mut self) -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.reject_coverage_selector.to_owned().unwrap();
		self.get_el(selector).await
	}

	async fn get_account_list_el(&mut self) -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.account_list.to_owned().unwrap();
		self.get_el(selector).await
	}

	async fn get_buy_now_el(&mut self) -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.buy_now.to_owned().unwrap();
		self.get_el(selector).await
	}

	async fn get_submit_order_el(&mut self) -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.submit_order.to_owned().unwrap();
		self.get_el(selector).await
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

	async fn get_login_el(&mut self)  -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.account_list.to_owned().unwrap();
		self.get_el(selector).await
	}

	async fn get_price_el(&mut self)-> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.price.to_owned().unwrap();
		self.get_el(selector).await
	}

	async fn get_sign_in_form(&mut self)  -> Option<Form> {
		let selector = JOINEM_CONFIG.amazon.selectors.sign_in_form.to_owned().unwrap();
		self.get_form(selector).await
	}

	async fn get_email_el(&mut self)  -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.email.to_owned().unwrap();
		self.get_el(selector).await
	}

	async fn get_password_el(&mut self)  -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.password.to_owned().unwrap();
		self.get_el(selector).await
	}

//         let current_url = c2.current_url().await?;
//         let path = current_url.path();
//         // "https://www.amazon.com/ap/signin"
//         info!("{} Logging in!", url.name);
//         let sign_in_path = "/ap/signin";
//         if path.eq(sign_in_path) {
//         }
//
//         //https://www.amazon.com/gp/buy/spc/handlers/display.html?hasWorkingJavascript=1
//         // let cart_path = "/ap/signin";
//         //
//         let cart_path = "/gp/buy/spc/handlers/display.html";
//         if path.eq(cart_path) {
//           debug!("It took us to shopping cart!");
//         } else { // We're still on product page with pop up
//           // confirm
//           confirm_buy_now(& mut c2).await;
//         }

	async fn get_turbo_iframe_el(&mut self)  -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.turbo_frame.to_owned().unwrap();
		self.get_el(selector).await
	}

	async fn get_turbo_checkout_el(&mut self)  -> Option<Element> {
		let selector = JOINEM_CONFIG.amazon.selectors.turbo_checkout.to_owned().unwrap();
		self.get_el(selector).await
	}

  pub async fn auto_login(&mut self, elements: &AmazonElements) -> Action {
		if elements.account_list_el.is_some() {
			return End
		} else if elements.login_el.is_some() {
      debug!("AMAZONSIGNIN");
      Click(elements.login_el.clone().unwrap())
		} else if elements.sign_in_form.is_some() {
			Submit(elements.sign_in_form.clone().unwrap())
		} else {
      debug!("SLEEP");
      Wait
    }
  }

	pub async fn auto_purchase(&mut self, elements: &AmazonElements, item: Item) -> Action {
		let current_url = self.client.current_url().await.unwrap();
		let path = current_url.path();

		if elements.sign_in_form.is_some() {
			Submit(elements.sign_in_form.clone().unwrap())
		} else if elements.reject_coverage_el.is_some() {
			info!("COVERAGEREJECT\t{}", item.name);
			Click(elements.reject_coverage_el.as_ref().unwrap().clone())
		} else if elements.turbo_checkout_el.is_some() { 
			info!("TURBOCHECKOUT\t{}", item.name);
			Click(elements.turbo_checkout_el.clone().unwrap())
		} else if elements.submit_order_el.is_some() {
			info!("ORDERSUBMIT\t{}", item.name);
			Click(elements.submit_order_el.clone().unwrap())
		} else if elements.price.is_some() 
				&& elements.buy_now_el.is_some() 
				&& elements.price.unwrap() <= item.max_price {
				info!("AFFORDABLE\t{}", item.name);
				Click(elements.buy_now_el.clone().unwrap())
		} else {
      debug!("SLEEP");
			Wait
		}
	}
}

impl BaseBot for Bot {
  fn client(&mut self) -> &mut Client { &mut self.client }
  fn item(&mut self) -> &mut Option<Item> { &mut self.item }
}

