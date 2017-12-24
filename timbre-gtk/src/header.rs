use gtk::{HeaderBar, HeaderBarExt, Stack, StackSwitcher, StackSwitcherExt};

pub struct Header {
    pub container: HeaderBar,
    stack_switch: StackSwitcher,
}

impl Header {
    pub fn new() -> Header {
        let container = HeaderBar::new();
        container.set_title("Timbre");
        container.set_show_close_button(true);

        let stack_switch = StackSwitcher::new();

        container.pack_start(&stack_switch);

        Header {
            container,
            stack_switch,
        }
    }

    pub fn set_stack(&self, stack: &Stack) {
        self.stack_switch.set_stack(stack);
    }
}
