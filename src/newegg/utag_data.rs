use fantoccini::{Client};

pub fn udata_string(utag_data: &serde_json::Value, name: &str) -> Option<String> {
	let element: String = utag_data[name].as_str().unwrap_or("").to_owned();
	if element.eq("") {
		return None;
	} 

  Some(element)
}

pub fn udata_array(utag_data: &serde_json::Value, name: &str) -> Option<String> {
  match utag_data[name].as_array() {
    Some(array) => {
      let element = array[0].as_str().unwrap_or("");
      let element = if element.eq("") {
        None
      } else {
        Some(element.to_owned())
      };
      element
    },
    None => { None }
  }
}

pub fn udata_array_f32(utag_data: &serde_json::Value, name: &str) -> Option<f32> {
  match utag_data[name].as_array() {
    Some(array) => {
      let element = array[0].as_str().unwrap_or("");
      let element = if element.eq("") {
        None
      } else {
        Some(element.parse::<f32>().unwrap())
      };
      element
    },
    None => {
      None
    }
  }
}

pub fn udata_array_bool(utag_data: &serde_json::Value, name: &str) -> Option<bool> {
  match utag_data[name].as_array() {
    Some(array) => {
      let element = array[0].as_str().unwrap_or("");
      let element = if element.eq("") {
        None
      } else {
        Some(element.eq("1"))
      };
      element
    },

    None => {return None;}
  }
}

pub struct Utag_Data {
  pub user_name: Option<String>,
  pub page_name: Option<String>,
  pub product_sale_price: Option<f32>,
  pub product_instock: Option<bool>
}

pub async fn newegg_utag_data(mut client: & mut Client) -> Result<Utag_Data, fantoccini::error::CmdError> {
	let utag_data = client.execute("return utag_data", vec![]).await.unwrap();

	let user_name: Option<String> =  udata_string(&utag_data, "user_name");
	let page_name: Option<String> = udata_string(&utag_data, "page_name");

	let product_sale_price = udata_array_f32(&utag_data, "product_sale_price");
	let product_instock = udata_array_bool(&utag_data,"product_instock");

// "NewProductDetail" "ShoppingCart"
//"cart_grand_total": "29.99"
// site_state: 0

  let utag_data = Utag_Data{
    user_name,
    page_name,
    product_sale_price,
    product_instock
  };

  // debug!("UTAGDATA {:?}", utag_data);
  Ok(utag_data)
}
