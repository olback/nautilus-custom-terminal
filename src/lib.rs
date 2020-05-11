use nautilus_extension::{nautilus_module, NautilusModule};
use gobject_sys::GTypeModule;
use glib_sys::GType;
use libc::c_int;
use lazy_static::lazy_static;

mod config;
mod custom_terminal_menu_provider;
mod utils;
mod open_terminal;
use custom_terminal_menu_provider::CustomTerminalMenuProvider;
use config::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

fn init(module: *mut GTypeModule) -> GType {

    println!("Initializing Custom Terminal {}", env!("CARGO_PKG_VERSION"));

    NautilusModule::new(module, "CustomTerminal")
        .add_menu_provider(CustomTerminalMenuProvider::new())
        .register()

}

nautilus_module!(init);
