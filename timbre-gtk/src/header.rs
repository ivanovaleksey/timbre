use gtk::{self, HeaderBarExt, StackSwitcherExt};

pub struct Header {
    pub container: gtk::HeaderBar,
    stack_switch: gtk::StackSwitcher,
}

impl Header {
    pub fn new() -> Header {
        let container = gtk::HeaderBar::new();
        let stack_switch = gtk::StackSwitcher::new();

        container.set_custom_title(&stack_switch);
        container.set_show_close_button(true);

        Header {
            container,
            stack_switch,
        }
    }

    pub fn set_stack(&self, stack: &gtk::Stack) {
        self.stack_switch.set_stack(stack);
    }
}
