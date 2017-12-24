use gtk::{self, ContainerExt, GtkWindowExt, Inhibit, WidgetExt};

pub fn new(app: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(app);

    window.set_title("Timbre");
    window.set_wmclass("app-name", "Timbre");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1150, 650);

    let window_1 = window.clone();
    window.connect_delete_event(move |_, _| {
        window_1.destroy();
        Inhibit(false)
    });

    window
}
