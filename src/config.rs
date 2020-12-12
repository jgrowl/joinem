
use std::env;
use base_config::{Config, File, FileFormat, Environment, ConfigError};
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


use serde_derive::Deserialize;

// pub type Item = (String, f32, String);
#[derive(Clone, Deserialize, Debug)]
pub struct Item {
  pub name: String, 
  pub max_price: f32, 
  pub url: String
} 

use std::sync::{Arc, Mutex};



#[derive(Debug, Deserialize)]
pub struct JoinemConfig {


pub username: String ,
pub password: String ,
pub chrome_user_data: String, 
pub data: String,
pub items: Vec<Item>

  // settings: HashMap<String, String>,
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
	pub fn new() -> Result<Self, ConfigError> {
  // pub fn new() -> JoinemConfig {

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

    // let settings =  settings.try_into::<HashMap<String, String>>().unwrap();

		let data = settings.get::<String>("data").expect("Data directory must be set!");
		std::fs::create_dir_all(data).expect("Failed to create data directory!");

		settings.try_into()
  }


  pub fn create_data_folder(&self, out_dir: String) {
    // std::fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");
    // self.settings.get("data").unwrap().clone()
    // fs::create_dir_all(&out_dir_base).expect("Failed to create directory!");
    fs::create_dir_all(&out_dir).expect("Failed to create directory!");

    let options = CopyOptions::new(); //Initialize default values for CopyOptions
    // options.mirror_copy = true; // To mirror copy the whole structure of the source directory
    //

    let default = self.chrome_user_data.to_owned();
    debug!("Chrome user_data path set to {}", &default);
// copy(default, &out_dir, &options).expect("uho");
//
    copy_dir_all(default, &out_dir).expect("Failed to copy chrome data dir");
  }

  fn get_unused_data_dirs(&self) -> Vec<String> {
    // scan folder structure to see if any folders are there
    let paths = fs::read_dir(self.data.to_owned()).unwrap();
    let paths: Vec<String> = paths.into_iter().map(|x| 
      x.unwrap().path().to_str().to_owned().unwrap().to_owned()
    ).collect();

    let b: HashSet<String> = paths.iter().cloned().collect();

    let mut data_dirs = get_data_dirs();

    let a: HashSet<String> = data_dirs.clone().iter().cloned().collect();

    let diff1: HashSet<_> = a.symmetric_difference(&b).collect();
    let mut v = Vec::from_iter(diff1.iter().map(|x| (*x).to_owned()));
    v
  }

  pub fn find_or_create_data_folder(&self) -> String {
    let mut v = self.get_unused_data_dirs();

    let out_dir = if v.len() > 0 {
      let out_dir= v.pop().unwrap().to_owned().to_string();
      debug!("Reusing data_dir `{}`", out_dir);
      {
        let mut data_dirs = get_data_dirs();
        data_dirs.push(out_dir.clone());
      }
    out_dir
    } else { // if there are none, then create one
      let out_dir = format!("{}/{}", self.data.to_owned(), random_string());

    {
      let mut data_dirs = get_data_dirs();
      data_dirs.push(out_dir.clone());
    }

      self.create_data_folder(out_dir.to_owned().to_string());

      out_dir
    };


    out_dir
  }
}

#[cfg(test)]
mod tests {
  use crate::config::JoinemConfig;
    #[test]
    fn it_works2() {
			let config = JoinemConfig::new().unwrap();
			let data_dir = config.items.to_owned().pop();
			println!("YO: {:?}", data_dir);
			assert!(config.data.to_owned() == "".to_string());
    }

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
