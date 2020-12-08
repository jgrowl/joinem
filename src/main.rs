use fantoccini::{Client, Locator};
use tokio::time::delay_for;
use async_std::future;

use std::process;
use std::time::{Duration};

// let's set up the sequence of steps we want the browser to take
#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut c = Client::new("http://localhost:4444").await.expect("failed to connect to WebDriver");

    // let amazon_sign_in_url = "https://www.amazon.com/sign/s?k=sign+in";
    let amazon_sign_in_url = "https://www.amazon.com/ap/signin?_encoding=UTF8&openid.assoc_handle=usflex&openid.claimed_id=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.identity=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.mode=checkid_setup&openid.ns=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0&openid.ns.pape=http%3A%2F%2Fspecs.openid.net%2Fextensions%2Fpape%2F1.0&openid.pape.max_auth_age=0&openid.return_to=https%3A%2F%2Fwww.amazon.com%2Fgp%2Fcss%2Fhomepage.html%3Fie%3DUTF8%26%252AVersion%252A%3D1%26%252Aentries%252A%3D0";

    // "ap_email"

    c.goto(amazon_sign_in_url).await?;
    // let url = c.current_url().await?;
    //
    //
    let search_form = c.form(Locator::Css("form[name='signIn']")).await?;
    let mut search_input = c.find(Locator::Id("ap_email")).await?;
    let username = "username";
    search_input.send_keys(username).await?;
    search_form.submit().await?;
    //
    // 

    let search_form = c.form(Locator::Css("form[name='signIn']")).await?;
    let mut search_input = c.find(Locator::Id("ap_password")).await?;
    let password = "password";
    search_input.send_keys(password).await?;
    search_form.submit().await?;
    ////

    let mut e = c.wait_for_find(Locator::Id("nav-link-accountList")).await?;
    println!("Logged in!");


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

    // out of stock
    // let url = "https://www.amazon.com/dp/B07XPC9B55/ref=twister_B08LYZMK9C?_encoding=UTF8&psc=1";

    // coffee, it works!
    // let url = "https://www.amazon.com/gp/product/B078TN99F9";

    // subscribe and save test
    // let url = "https://www.amazon.com/gp/product/B003SGHSCG?pf_rd_r=PJCXN2B304AV890R0GP2";

    let url = "https://www.amazon.com/AMD-Ryzen-5950X-32-Thread-Processor/dp/B0815Y8J9N";
    // let url = "https://www.amazon.com/AMD-Ryzen-5900X-24-Thread-Processor/dp/B08164VTWH";


    let tab2 = c.new_window(true).await.unwrap();
    //
    // // c.switch_to_window(win2.handle);
    // let windows = c.windows().await.unwrap();
    // // println!("{}", windows.len());
    // // let tab2 = windows.get(1).unwrap().clone();
    let tab2 = webdriver::common::WebWindow(tab2.handle);
    c.switch_to_window(tab2).await;
    c.goto(url).await?;

    // let mut buy_now = c.wait_for_find(Locator::Id("buy-now-button"));
    let mut buy_now = c.wait_for_find(Locator::Id("buyNow"));
    
    let dur = Duration::from_secs(5);
    match future::timeout(dur, buy_now).await {
      Ok(button) => {
        println!("yay");
        // Need to wait some time or else will take to shopping cart
        delay_for(Duration::from_millis(5000)).await;

        button.unwrap().click().await;


        // delay_for(Duration::from_millis(3000)).await;
        // let form = c.form(Locator::Id("place-order-form")).await;
        // form.unwrap().submit().await;


        // let active = c.active_element().await.unwrap();
        // println!("{:?}", active.element);
        // enter_frame
        let frame_id = "turbo-checkout-iframe";
        let frame = c.wait_for_find(Locator::Id(frame_id)).await;
        frame.unwrap().enter_frame().await;

        // // let place_order_id = "turbo-cel-place-order-button";
        // let place_order_id = "turbo-checkout-pyo-button";
        let place_order_id = "turbo-checkout-place-order-button";
        let mut place_order = c.wait_for_find(Locator::Id(place_order_id)).await;
        place_order.unwrap().click().await;

        // wait for confirm

        println!("You got it dude!");
        // process::exit(0x0100);
      },
      Err(e) => {
        println!("boo");

        println!("refreshing...");
        // c.refresh().await;
      }
    }




    delay_for(Duration::from_millis(20000)).await;
    // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");
    // // click "Foo Lake"
    // c.find(Locator::LinkText("Foo Lake")).await?.click().await?;
    //
    // let url = c.current_url().await?;
    // assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");
    //


    c.close().await
}



// #![deny(warnings)]
// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
//
// mod routes;
//
// use std::error::Error;
// use std::convert::Infallible;
//
// use futures::future::TryFutureExt;
//
// use hyper::server::Server;
//
// use listenfd::ListenFd;
//
// use tokio;
// use tokio::net::TcpListener;
// use tokio::sync::oneshot;
// use tokio::sync::oneshot::{Sender, Receiver};
//
//
// use warp::Filter;
// extern crate futures;
// extern crate warp;
//
// extern crate listenfd;
//
// // fn main() -> Result<(), Box<dyn Error>> {
// //  let mut runtime = tokio::runtime::Builder::new().core_threads(1).enable_io().build()?;
// //
// //
// //
// //     runtime.block_on(async {
//
// #[tokio::main]
// async fn main() {
//   println!("Started...");
//
//   pretty_env_logger::init();
//
//   let routes = routes::get_routes();
//
//
//   let mut listenfd = ListenFd::from_env();
//   let (tx, rx) = oneshot::channel();
//     // if listenfd doesn't take a TcpListener (i.e. we're not running via
//     // the command above), we fall back to explicitly binding to a given
//     // host:port.
//     // let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
//     //
//   if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
//
//       // hyper let's us build a server from a TcpListener (which will be
//       // useful shortly). Thus, we'll need to convert our `warp::Filter` into
//       // a `hyper::service::MakeService` for use with a `hyper::server::Server`.
//       let svc = warp::service(routes);
//
//       let make_svc = hyper::service::make_service_fn(|_: _| {
//         // the clone is there because not all warp filters impl Copy
//         let svc = svc.clone();
//         async move { Ok::<_, Infallible>(svc) }
//       });
//
//       // let server = Server::from_tcp(l).unwrap();
//       let server = Server::from_tcp(l).unwrap();
//       server.serve(make_svc).await.unwrap();
//     } else {
//
//       let (addr, server) = warp::serve(routes)
//         // let (addr, server) = warp::serve(make_svc)
//         .bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
//           rx.await.ok();
//         });
//
//       // Spawn the server into a runtime
//       tokio::task::spawn(server);
//
//       // Later, start the shutdown...
//       let _ = tx.send(());
//
//
//       // let server = Server::bind(&([127, 0, 0, 1], 3030).into());
//       // server.serve(make_svc).await.unwrap();
//     };
//
//     //
//     // });
//     //
//     // Ok(())
// }
