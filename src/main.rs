//! # remarko
//! 
//! A small webview window for previewing makdown and localhost.
//! 
//! ## Instalation
//! ### Cargo
//! `cargo install remarko`
//! ## Usage
//! Use the command remarko:<br>
//! `remarko <URL OR MARKDOWN FILE>`<br>
//! **Examples**:<br>
//!  `remarko https://docs.rs/wry` <br>
//!  `remarko http://localhost:3000` <br>
//!  `remarko README.md`
//! > If you give it a **markdown file** it **will** automaticly **start a live server**.
//! ## Contributions
//! **Issues**, **Ideas** and **Contributions** are [welcome]()!
use std::net::{SocketAddrV4, Ipv4Addr};

use markdown::{create_markdown_server};
use window::create_window;
use wry::{
  application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder, dpi::{PhysicalSize, PhysicalPosition}, menu::MenuId,
  },
  webview::WebViewBuilder,
};
use clap::Parser;
use args::*;


#[cfg(target_os = "macos")]
#[macro_use(msg_send, sel)]
extern crate objc;


mod args;
mod window;
mod markdown;


fn main() -> wry::Result<()> {
    let args = Args::parse();
    let mut server = Option::None;
    let url = if args.url.ends_with(".md") {
      server = Option::Some(create_markdown_server(args.url.as_str()));
      format!("http://localhost:3456/")
    } else {
      args.url
    };
    
    let event_loop = EventLoop::new();
    
    let window = create_window(&event_loop);
    let _webview = WebViewBuilder::new(window?)?
      .with_url(url.as_str())?
      .build()?;
    

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
  
      match event {
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => {
          println!("{:#?}", server);
          *control_flow = ControlFlow::Exit
        },
        Event::MenuEvent { window_id, menu_id, origin, ..} => {
          if MenuId::new("remarko.reload").0==menu_id.0 {
            _webview.load_url(_webview.url().as_str());
          }
          if MenuId::new("remarko.reset").0==menu_id.0 {
            _webview.load_url(url.as_str());
          }
        }
        _ => (),
      }
    });
}