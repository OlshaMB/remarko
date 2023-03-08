use std::net::{SocketAddrV4, Ipv4Addr};
use markdown::create_markdown_server;
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
    let url = if args.url.ends_with(".md") {
      format!("http://localhost:3456/{}", args.url)
    } else {
      args.url
    };
    create_markdown_server();
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
        } => *control_flow = ControlFlow::Exit,
        Event::MenuEvent { window_id, menu_id, origin, ..} => {
          if MenuId::new("marko.reload").0==menu_id.0 {
            _webview.load_url(_webview.url().as_str());
          }
          if MenuId::new("marko.reset").0==menu_id.0 {
            _webview.load_url(url.as_str());
          }
        }
        _ => (),
      }
    });
}