use fantoccini::{Client, Locator, Element, Form};
use super::Bot;
use crate::JOINEM_CONFIG;

use std::str::FromStr;


	// async fn confirm_buy_now(c: & mut Client) -> Result<(), fantoccini::error::CmdError> {
	//   let frame_id = "turbo-checkout-iframe";
	//   let frame = c.wait_for_find(Locator::Id(frame_id)).await;
	//   frame.unwrap().enter_frame().await;
  //
	//   // let form = c.form(Locator::Id("place-order-form")).await;
	//   // form.unwrap().submit().await;
  //
	//   // // let place_order_id = "turbo-cel-place-order-button";
	//   // let place_order_id = "turbo-checkout-pyo-button";
	//   let place_order_id = "turbo-checkout-place-order-button";
	//   let place_order = c.wait_for_find(Locator::Id(place_order_id)).await;
	//   place_order.unwrap().click().await;
  //
	//   info!("You got it dude!");
  //
	//   Ok(())
	// }


pub struct AmazonElements {
  pub reject_coverage_el: Option<Element>,
	pub account_list_el: Option<Element>,
	pub buy_now_el: Option<Element>,
	pub affordable_buy_now_el: Option<Element>,
	pub login_el: Option<Element>,
	pub price_el: Option<Element>,
	pub price: Option<f32>,
	pub sign_in_form: Option<Form>,
	pub email_el: Option<Element>,
	pub password_el: Option<Element>,
	pub turbo_iframe_el: Option<Element>,
	pub turbo_checkout_el: Option<Element>,
	pub submit_order_el: Option<Element>,
}

impl AmazonElements {
	pub async fn new(mut bot: &mut Bot) -> AmazonElements {
		let reject_coverage_el = bot.get_reject_coverage_el().await;
		let account_list_el = bot.get_account_list_el().await;
		let buy_now_el = bot.get_buy_now_el().await;
		let affordable_buy_now_el = bot.get_affordable_buy_now_el().await;

		let login_el = bot.get_login_el().await;

		let price_el = bot.get_price_el().await;
		let price = bot.get_price().await; 

		let sign_in_form = bot.get_sign_in_form().await;

		let email_el = bot.get_email_el().await;
		let password_el = bot.get_password_el().await;

		let submit_order_el = bot.get_submit_order_el().await;

		let mut turbo_checkout_el = None;
		let turbo_iframe_el = bot.get_turbo_iframe_el().await;
		if turbo_iframe_el.is_some() {
			let frame = turbo_iframe_el.clone().unwrap();
			frame.enter_frame().await;
			turbo_checkout_el = bot.get_turbo_checkout_el().await;
			bot.client.clone().enter_parent_frame().await;
		}

		let elements = AmazonElements{
			submit_order_el,
			reject_coverage_el,
			account_list_el,
			buy_now_el,
			affordable_buy_now_el,
			login_el,
			price_el,
			price,
			sign_in_form,
			email_el,
			password_el,
			turbo_iframe_el,
			turbo_checkout_el,
		};

		elements.fill().await;

		elements
	}

	pub async fn fill(&self) {
		match self.email_el.clone() {
			Some(mut email_el) => {
				let username = JOINEM_CONFIG.amazon.username.to_owned();
				self.email_el.clone().unwrap().send_keys(&username).await;
			},
			None => {}
		}

		match self.password_el.clone() {
			Some(mut password_el) => {
				let password = JOINEM_CONFIG.amazon.password.to_owned();
				password_el.send_keys(&password).await;
			},
			None => {}
		}
	}
}

