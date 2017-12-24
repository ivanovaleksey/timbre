use gtk::{self, BoxExt, LabelExt, StackExt};

pub struct Content {
    pub container: gtk::Box,
    stack: gtk::Stack,
}

impl Content {
    pub fn new() -> Content {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let stack = gtk::Stack::new();
        let checkbutton = gtk::CheckButton::new_with_label("Click me!");
        stack.add_titled(&checkbutton, "check", "Check Button");

        let label_1 = gtk::Label::new("Test label");
        stack.add_titled(&label_1, "label_1", "A label #1");

        let label_2 = gtk::Label::new("");
        label_2.set_markup("<big>A fancy label</big>");
        stack.add_titled(&label_2, "label_2", "A label #2");

        container.pack_start(&stack, true, true, 0);

        Content { container, stack }
    }

    pub fn get_stack(&self) -> &gtk::Stack {
        &self.stack
    }
}
