use std::net::{SocketAddrV4, Ipv4Addr};
use wry::{
  application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
  },
  webview::WebViewBuilder,
};
use clap::Parser;
mod args;
use args::*;

#[cfg(target_os = "macos")]
#[macro_use(msg_send, sel)]
extern crate objc;

fn main() -> wry::Result<()> {
    let args = Args::parse();

    let event_loop = EventLoop::new();
    let mut window_builder = WindowBuilder::new().with_decorations(false);
    
    let window = window_builder.build(&event_loop)?;
    #[cfg(target_os = "macos")] {
        use wry::application::platform::macos::WindowExtMacOS;
        use cocoa::appkit::NSWindow;
        use cocoa::appkit::NSWindowStyleMask;
        use cocoa::appkit::NSWindowButton::NSWindowFullScreenButton;
        
        let ns_window = window.ns_window() as cocoa::base::id;
        unsafe {
            ns_window.setStyleMask_(
                NSWindowStyleMask::NSResizableWindowMask |
                 NSWindowStyleMask::NSClosableWindowMask | 
                 NSWindowStyleMask::NSFullSizeContentViewWindowMask | 
                 NSWindowStyleMask::NSTitledWindowMask
                );
            ns_window.setTitleVisibility_(cocoa::appkit::NSWindowTitleVisibility::NSWindowTitleHidden);
            ns_window.setTitlebarAppearsTransparent_(1);
            ns_window.setMovableByWindowBackground_(1);
            {
                use objc::sel_impl;
                let _: () = msg_send![ns_window.standardWindowButton_(cocoa::appkit::NSWindowButton::NSWindowMiniaturizeButton), setHidden: 1];
                let _: () = msg_send![ns_window.standardWindowButton_(cocoa::appkit::NSWindowButton::NSWindowZoomButton), setHidden: 1];
                let _: () = msg_send![ns_window.standardWindowButton_(cocoa::appkit::NSWindowButton::NSWindowCloseButton), setHidden: 1];
            }
        }
    }
    let _webview = WebViewBuilder::new(window)?
      .with_url(args.url.as_str())?
      .build()?;
    

    event_loop.run(move |event, _, control_flow| {
      *control_flow = ControlFlow::Wait;
  
      match event {
        Event::WindowEvent {
          event: WindowEvent::CloseRequested,
          ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
      }
    });
}