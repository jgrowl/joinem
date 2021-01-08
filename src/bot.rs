use fantoccini::{Client, Locator, Element, Form};

use async_trait::async_trait;

use crate::Item;

#[async_trait]
pub trait Bot {
  fn client(&mut self) -> &mut Client;
  fn item(&mut self) -> &mut Option<Item>;

	fn item_url(&mut self) -> Option<String> {
		if self.item().is_none() {
			return None
		};

		let item = self.item().clone().unwrap();
		Some(item.url)
	}

  async fn get_el(&mut self, selector: String) -> Option<Element> {
    let mut element = self.client().find(Locator::Css(&selector)).await;
    match element {
      Ok(element) => Some(element),
      Err(e) => None
    }
  }

  async fn get_form(&mut self, selector: String) -> Option<Form> {
    let mut form = self.client().form(Locator::Css(&selector)).await;
    match form {
      Ok(form) => Some(form),
      Err(e) => None
    }
  }

	async fn get_el_with_text(&mut self, selector: String, content: String) -> Option<Element> {
		let mut element = self.client().find(Locator::Css(&selector)).await;
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

  async fn refresh(&mut self) {
      self.client().refresh().await;
  }

	async fn goto(&mut self) {
		match self.item_url() {
			Some(url) => {
				self.client().goto(&url).await;
			},
			None => {}
		}
  }

}

