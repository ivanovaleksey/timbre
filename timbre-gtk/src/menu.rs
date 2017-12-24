use gio::{self, MenuExt};
use gtk::{self, GtkApplicationExt};

pub fn new(app: &gtk::Application) -> gio::Menu {
    let menu = gio::Menu::new();
    let menu_bar = gio::Menu::new();
    let more_menu = gio::Menu::new();
    let switch_menu = gio::Menu::new();
    let settings_menu = gio::Menu::new();
    let submenu = gio::Menu::new();

    menu.append("Quit", "app.quit");

    switch_menu.append("Switch", "app.switch");
    menu_bar.append_submenu("_Switch", &switch_menu);

    settings_menu.append("Sub another", "app.sub_another");
    submenu.append("Sub sub another", "app.sub_sub_another");
    submenu.append("Sub sub another2", "app.sub_sub_another2");
    settings_menu.append_submenu("Sub menu", &submenu);
    menu_bar.append_submenu("_Another", &settings_menu);

    more_menu.append("About", "app.about");
    menu_bar.append_submenu("?", &more_menu);

    app.set_app_menu(&menu);
    app.set_menubar(&menu_bar);

    menu
}
