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
        window.add(&content.container);

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
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 500);

    let window_1 = window.clone();
    window.connect_delete_event(move |_, _| {
        window_1.destroy();
        Inhibit(false)
    });

    window
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
