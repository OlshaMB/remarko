use std::str::FromStr;

use wry::application::{window::{WindowBuilder, Window}, event_loop::EventLoop, menu::{MenuBar, MenuItemAttributes, MenuId}, accelerator::Accelerator};

pub fn create_window(event_loop: &EventLoop<()>) -> wry::Result<Window> {
    let mut window_builder = WindowBuilder::new()
      .with_decorations(false)
      .with_resizable(true);
    window_builder = window_builder.with_menu(create_menu());
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

    #[cfg(target_os="windows")]
    {
      window.set_decorations(true);
    }
    wry::Result::Ok(window)
}
fn create_menu() -> MenuBar {
    let mut root = MenuBar::new();
    let mut marko = MenuBar::new();
    marko.add_item(
        MenuItemAttributes::new("Reload")
            .with_accelerators(
                &Accelerator::from_str("cmd+r").unwrap()
            )
            .with_id(MenuId::new("marko.reload"))
    );
    marko.add_item(
        MenuItemAttributes::new("Reset")
            .with_accelerators(
                &Accelerator::from_str("shift+cmd+r").unwrap()
            )
            .with_id(MenuId::new("marko.reset"))
    );
    marko.add_native_item(wry::application::menu::MenuItem::Minimize);
    marko.add_native_item(wry::application::menu::MenuItem::Hide);
    marko.add_native_item(wry::application::menu::MenuItem::CloseWindow);
    marko.add_native_item(wry::application::menu::MenuItem::Quit);
    root.add_submenu("marko", true, marko);
    root
}