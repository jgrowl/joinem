#![deny(warnings)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod routes;

use std::error::Error;
use std::convert::Infallible;

use futures::future::TryFutureExt;

use hyper::server::Server;

use listenfd::ListenFd;

use tokio;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Sender, Receiver};


use warp::Filter;
extern crate futures;
extern crate warp;

extern crate listenfd;

// fn main() -> Result<(), Box<dyn Error>> {
//  let mut runtime = tokio::runtime::Builder::new().core_threads(1).enable_io().build()?;
//
//
//
//     runtime.block_on(async {

#[tokio::main]
async fn main() {
  println!("Started...");

  pretty_env_logger::init();

  let routes = routes::get_routes();


  let mut listenfd = ListenFd::from_env();
  let (tx, rx) = oneshot::channel();
    // if listenfd doesn't take a TcpListener (i.e. we're not running via
    // the command above), we fall back to explicitly binding to a given
    // host:port.
    // let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    //
  if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {

      // hyper let's us build a server from a TcpListener (which will be
      // useful shortly). Thus, we'll need to convert our `warp::Filter` into
      // a `hyper::service::MakeService` for use with a `hyper::server::Server`.
      let svc = warp::service(routes);

      let make_svc = hyper::service::make_service_fn(|_: _| {
        // the clone is there because not all warp filters impl Copy
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
      });

      // let server = Server::from_tcp(l).unwrap();
      let server = Server::from_tcp(l).unwrap();
      server.serve(make_svc).await.unwrap();
    } else {

      let (addr, server) = warp::serve(routes)
        // let (addr, server) = warp::serve(make_svc)
        .bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
          rx.await.ok();
        });

      // Spawn the server into a runtime
      tokio::task::spawn(server);

      // Later, start the shutdown...
      let _ = tx.send(());


			// let server = Server::bind(&([127, 0, 0, 1], 3030).into());
			// server.serve(make_svc).await.unwrap();
    };

    //
    // });
    //
    // Ok(())
}
