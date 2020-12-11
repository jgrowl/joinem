
use std::env;
use base_config::{Config, File, FileFormat, Environment};
use std::collections::HashMap;
use log::{info, warn, debug};
use std::{io, fs};
use std::path::Path;

extern crate fs_extra;
use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use crate::util::{copy_dir_all, random_string};

use std::fs::read_dir;

use std::collections::HashSet;
use std::iter::FromIterator;
// use crate::DATA_DIRS;
use crate::get_data_dirs;

pub type Item = (String, f32, String);

use std::sync::{Arc, Mutex};


pub struct JoinemConfig {
  settings: HashMap<String, String>,
  // data_dirs: Vec<String>
}
// use std::ops::{DerefMut, Deref};
//
// impl<T> Deref for JoinemConfig<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.value
//     }
// }
//
// impl<T> DerefMut for JoinemConfig<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.value
//     }
// }

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

  // pub fn next_available_data_dir(&mut self) -> String {
  //   // let out_dir_base = self.data();
  //   self.find_or_create_data_folder()
  // }

  pub fn create_data_folder(&self, out_dir: String) {
    // std::fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");
    // self.settings.get("data").unwrap().clone()
    // fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");
  fs::create_dir_all(&out_dir).expect("Failed to create directory!");


  let options = CopyOptions::new(); //Initialize default values for CopyOptions
// options.mirror_copy = true; // To mirror copy the whole structure of the source directory
//

// copy source/dir1 to target/dir1
// let default = "~/Library/Caches/Google/Chrome/Default";
let default = self.chrome_user_data();
debug!("Chrome user_data path set to {}", &default);
// println!("{}", default);
// let default = "/Users/jon/Library/Caches/Google/Chrome";
// copy(default, &out_dir, &options).expect("uho");
//

  // this actually copies
  copy_dir_all(default, &out_dir).expect("Failed to copy chrome data dir");

  // println!("outdir: {}", out_dir);
  // copy_dir_all("~/Library/Caches/Google/Chrome/Default", out_dir);
  //
  }


  pub fn find_or_create_data_folder(&self) -> String {
    // scan folder structure to see if any folders are there
    let paths = fs::read_dir(self.data()).unwrap();
    let paths: Vec<String> = paths.into_iter().map(|x| 
      x.unwrap().path().to_str().to_owned().unwrap().to_owned()
    ).collect();

    let b: HashSet<String> = paths.iter().cloned().collect();

    // let mut data_dirs = Arc::clone(&DATA_DIRS);
    // let mut data_dirs = data_dirs.lock().unwrap();
    let mut data_dirs = get_data_dirs();

    let a: HashSet<String> = data_dirs.clone().iter().cloned().collect();

    let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
    let mut v = Vec::from_iter(diff1.iter());

    let out_dir = if v.len() > 0 {
      let out_dir= v.pop().unwrap().to_owned().to_string();

      // data_dirs.push(out_dir.clone());

      out_dir
    } else { // if there are none, then create one
      let out_dir = format!("{}/{}", self.data(), random_string());

      self.create_data_folder(out_dir.to_owned().to_string());

      // let mut data_dirs = Arc::clone(&DATA_DIRS);
      // let mut data_dirs = data_dirs.lock().unwrap();

      out_dir
    };

    data_dirs.push(out_dir.clone());
    out_dir
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

#[cfg(test)]
mod tests {
  use crate::config::JoinemConfig;
    #[test]
    fn it_works() {
      // let config = JoinemConfig::new();
      // let data_dir = config.find_or_create_data_folder();
      // println!("YO: {}", data_dir);
      // // assert!(config.data() == "".to_string());
      //
      // let data_dir = config.find_or_create_data_folder();
      // println!("YO: {}", data_dir);
      // //
      // assert!(false);
    }
}
