use nautilus_extension::{nautilus_module, NautilusModule};
use gobject_sys::GTypeModule;
use glib_sys::GType;
use libc::c_int;

mod config;
mod custom_terminal_menu_provider;
mod utils;
use custom_terminal_menu_provider::CustomTerminalMenuProvider;

fn init(module: *mut GTypeModule) -> GType {

    println!("Initializing Custom Terminal {}", env!("CARGO_PKG_VERSION"));

    NautilusModule::new(module, "CustomTerminal")
        .add_menu_provider(CustomTerminalMenuProvider::new())
        .register()

}

nautilus_module!(init);
