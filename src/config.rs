
use std::env;
use base_config::{Config, File, FileFormat, Environment};
use std::collections::HashMap;

pub type Item = (String, f32, String);

pub struct JoinemConfig {
  settings: HashMap<String, String>
}

impl JoinemConfig {
  pub fn new() -> JoinemConfig { 

  let mut settings = base_config::Config::default();
  settings
    // Add in `./Settings.toml`
    .merge(base_config::File::with_name("Joinem").required(false)).unwrap()
    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    .merge(base_config::Environment::with_prefix("JOINEM")).unwrap();

    // // Add 'Settings.$(RUST_ENV).toml`
    // let name = format!("Settings.{}", env::var("env").unwrap_or("development".into()));
    // base_config.merge(File::new(&name, FileFormat::Toml).required(false)).unwrap();

    let settings =  settings.try_into::<HashMap<String, String>>().unwrap();
    JoinemConfig { settings: settings }
  }

  pub fn data(&self) -> String {
    self.settings.get("data").unwrap().clone()
  }

  pub fn password(&self) -> String {
    self.settings.get("password").unwrap().clone()
  }

  pub fn username(&self) -> String {
    self.settings.get("username").unwrap().clone()
  }

  pub fn chrome_user_data(&self) -> String {
    self.settings.get("chrome_user_data").unwrap().clone()
  }

  pub fn newegg_items() {

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

  pub fn items(&self) -> Vec<Item> {

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
