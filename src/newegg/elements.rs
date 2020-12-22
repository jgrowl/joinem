
use fantoccini::{Client, Locator, Element};
// use utag_data::Utag_Data;
// use crate::newegg::utag_data::Utag_Data;
use super::utag_data::Utag_Data;
use super::Bot;

pub struct NeweggElements {
  pub utag_data: Option<Utag_Data>,
  pub cvv_el: Option<Element>,
  pub username_el: Option<Element>,
  pub password_el: Option<Element>,
  pub sign_in_submit_el: Option<Element>,
  pub sign_in_el: Option<Element>,
  pub survey_el: Option<Element>,
  pub insurance_el: Option<Element>,
  pub  promotion_el:  Option<Element>,
  pub  continue_to_payment_el: Option<Element>,
  pub  view_cart_el: Option<Element>,
  pub  add_to_cart_el: Option<Element>,
  pub  secure_checkout_el: Option<Element>,
  pub card_number_el: Option<Element>,
  pub  save_el: Option<Element>,
  pub  ec_frame_el: Option<Element>, 
}

impl NeweggElements {
  pub async fn new(mut bot: &mut Bot) -> NeweggElements {
    let utag_data = super::utag_data::newegg_utag_data(&mut bot.client).await;
    let utag_data = match utag_data { 
      Ok(ok) => {Some(ok)},
      Err(err) => {None}
    };

    let cvv_el = bot.get_cvv_el_and_try_fill().await;
    let username_el = bot.get_username_el_and_try_fill().await;
    let password_el = bot.get_password_el_and_try_fill().await;

    let sign_in_submit_el = bot.get_sign_in_submit_el().await;
    let survey_el = bot.get_survey_el().await;
    let insurance_el = bot.get_insurance_el().await;
    let promotion_el = bot.get_promotion_el().await;
    let continue_to_payment_el = bot.get_continue_to_payment_el().await;
    let view_cart_el = bot.get_view_cart_el().await;
    let add_to_cart_el = bot.get_add_to_cart_el().await;
    let secure_checkout_el = bot.get_secure_checkout_el().await;


// div.nav-complex div.nav-complex-title.  <div class="nav-complex-title">Sign in / Register</div>
		let sign_in_el = bot.get_sign_in_el().await;


    let mut card_number_el = None;
    let mut save_el = None;
    let mut ec_frame_el = bot.get_ec_frame_el().await;
    if ec_frame_el.is_some() {
      let frame = ec_frame_el.clone().unwrap();

      frame.enter_frame().await;
      save_el = bot.get_save_el().await;

      card_number_el = bot.get_card_number_el_and_try_fill().await;
    }

    bot.client.clone().enter_parent_frame().await;

    NeweggElements{
      utag_data,
      cvv_el,
      username_el,
      password_el,
      sign_in_submit_el,
      survey_el,
      insurance_el,
      promotion_el,
      continue_to_payment_el,
      view_cart_el,
      add_to_cart_el,
      secure_checkout_el,
      card_number_el,
      save_el,
      ec_frame_el,
			sign_in_el,
    }
  }
}
