extern crate gio;
extern crate gtk;

use gio::{ApplicationExt, ApplicationExtManual};
use gtk::*;

use content::Content;
use header::Header;

mod content;
mod header;
mod menu;
mod window;

struct App {
    pub window: gtk::ApplicationWindow,
    pub menu: gio::Menu,
    pub header: Header,
    pub content: Content,
}

impl App {
    fn new(gtk_app: &Application) -> App {
        let window = window::new(gtk_app);

        let menu = menu::new(gtk_app);

        let header = Header::new();
        window.set_titlebar(&header.container);

        let content = Content::new();
        window.add(&content.container);

        {
            let stack = content.get_stack();
            header.set_stack(stack);
        }

        App {
            window,
            menu,
            header,
            content,
        }
    }

    fn init(gtk_app: &gtk::Application) {
        let my_app = App::new(gtk_app);
        my_app.window.show_all();
        my_app.window.activate();
        gtk_app.connect_activate(move |_| ());
    }
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
