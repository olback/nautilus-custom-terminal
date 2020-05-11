use nautilus_extension::{
    nautilus_menu_item_activate_cb,
    nautilus_menu_background_activate_cb,
    MenuProvider,
    FileInfo,
    MenuItem
};
use gtk_sys::GtkWidget;
use std::{path::PathBuf};
use super::{utils, CONFIG, open_terminal::open_terminal};
use gobject_sys::GObject;
use glib_sys::gpointer;

pub struct CustomTerminalMenuProvider;

impl CustomTerminalMenuProvider {

    pub fn new() -> Self {

        Self

    }

}

impl MenuProvider for CustomTerminalMenuProvider {

    fn get_file_items(&self, _window: *mut GtkWidget, files: &Vec<FileInfo>) -> Vec<MenuItem> {

        let mut menu_items = Vec::<MenuItem>::new();

        if files.len() == 1 && PathBuf::from(utils::fix_filename(files[0].get_uri())).is_dir() {

            let mut menu_item = MenuItem::new(
                "CustomTerminal::Open".into(),
                format!("Open in {}", CONFIG.name),
                format!("Open with terminal {}", CONFIG.name),
                None
            );

            menu_item.set_activate_cb(open_folder_clickded_cb);
            menu_items.push(menu_item);

        }

        menu_items

    }

    fn get_background_items(&self, _window: *mut GtkWidget, _current_folder: &FileInfo) -> Vec<MenuItem> {

        let mut menu_items = Vec::<MenuItem>::new();

        let mut menu_item = MenuItem::new(
            "CustomTerminal::Open".into(),
            format!("Open in {}", CONFIG.name),
            format!("Open with terminal {}", CONFIG.name),
            None
        );

        menu_item.set_activate_cb(open_background_clickded_cb);
        menu_items.push(menu_item);

        menu_items

    }

}

nautilus_menu_item_activate_cb!(open_folder_clickded_cb, open_folder_clicked);
nautilus_menu_background_activate_cb!(open_background_clickded_cb, open_background_clicked);

fn open_folder_clicked(files: Vec<FileInfo>) {

    if files.len() == 1 {

        let path = utils::fix_filename(files[0].get_uri());
        open_terminal(path);

    }

}

fn open_background_clicked(path: FileInfo) {

    let path = utils::fix_filename(path.get_uri());
    open_terminal(path);

}
