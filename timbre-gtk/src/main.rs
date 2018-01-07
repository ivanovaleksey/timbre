extern crate gio;
extern crate gtk;
extern crate timbre;

use gio::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::*;
use timbre::games::octaves;

use content::Content;

macro_rules! clone {
    ($($n:ident),+) => {
        $( let $n = $n.clone(); )+
    }
}

mod content;

struct App {
    pub window: gtk::ApplicationWindow,
    pub content: Content,
}

impl App {
    fn new(gtk_app: &gtk::Application) -> App {
        let config = octaves::Config::load();
        let controller = octaves::Controller::new_shared(config);

        let window = build_window(gtk_app);
        window.set_resizable(false);

        let header = gtk::HeaderBar::new();
        header.set_title("Timbre");
        header.set_show_close_button(true);
        window.set_titlebar(&header);

        let content = Content::new(&controller);
        let menu_bar = gtk::MenuBar::new();
        menu_bar.append(&build_game_menu(&window, &content));

        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        v_box.pack_start(&menu_bar, false, false, 0);
        v_box.pack_start(&content.container, true, true, 0);

        window.add(&v_box);

        App { window, content }
    }

    fn init(gtk_app: &gtk::Application) {
        let my_app = App::new(gtk_app);
        my_app.window.show_all();
        my_app.window.activate();
        gtk_app.connect_activate(move |_| ());
    }
}

fn build_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(app);

    window.set_title("Timbre");
    window.set_wmclass("app-name", "Timbre");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 500);

    let window_1 = window.clone();
    window.connect_delete_event(move |_, _| {
        window_1.destroy();
        Inhibit(false)
    });

    window
}

macro_rules! build_menu {
    ($menu:expr, [$( $item:expr ),*]) => {{
        let menu = gtk::MenuItem::new_with_mnemonic($menu);
        let submenu = gtk::Menu::new();
        $( submenu.append(&$item); )*
        menu.set_submenu(&submenu);
        menu
    }}
}

fn build_game_menu(window: &gtk::ApplicationWindow, content: &Content) -> gtk::MenuItem {
    let new = gtk::MenuItem::new_with_mnemonic("_New");
    let load = gtk::MenuItem::new_with_mnemonic("_Load");
    let save = gtk::MenuItem::new_with_mnemonic("_Save");
    let quit = gtk::MenuItem::new_with_mnemonic("_Quit");

    // TODO: prompt to save the game
    new.connect_activate({
        let revealer = content.revealer.clone();
        let start_btn = content.start_btn.clone();
        let ton_combo = content.tonality_combo.clone();
        move |_| {
            revealer.set_reveal_child(false);
            start_btn.set_sensitive(true);
            ton_combo.set_sensitive(true);
        }
    });

    // TODO: prompt to save the game
    quit.connect_activate({
        clone!(window);
        move |_| window.close()
    });

    build_menu!(
        "_Game",
        [new, load, save, gtk::SeparatorMenuItem::new(), quit]
    )
}

fn main() {
    let app = gtk::Application::new(
        "com.github.ivanovaleksey.timbre",
        gio::ApplicationFlags::empty(),
    ).expect("Failed to initialize GTK application");

    app.connect_startup(move |app| {
        App::init(app);
    });
    app.run(&[]);
}
