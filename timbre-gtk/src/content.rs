use gtk;
use gtk::prelude::*;

use timbre::games::octaves;

#[derive(Clone)]
pub struct Content {
    pub container: gtk::Box,
}

impl Content {
    pub fn new(controller: &octaves::SharedController) -> Content {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let menu_bar = gtk::MenuBar::new();

        let h_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let ton_combo = gtk::ComboBoxText::new();
        for tonality in octaves::note::TONALITIES.iter() {
            ton_combo.append_text(&format!("{:?}", tonality));
        }
        ton_combo.set_active(0);

        let start_btn = gtk::Button::new_with_label("Start");

        let statictics = Content::build_statistics_panel();

        let game_area = Content::build_game_area(controller);
        let revealer = gtk::Revealer::new();
        revealer.set_transition_type(gtk::RevealerTransitionType::Crossfade);
        revealer.set_transition_duration(2000);
        revealer.add(&game_area);

        let left_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let right_box = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let ton_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        ton_box.pack_start(&ton_combo, false, false, 0);
        ton_box.pack_start(&start_btn, false, false, 10);

        left_box.pack_start(&ton_box, false, false, 10);
        left_box.pack_start(&revealer, true, true, 0);
        right_box.pack_start(&statictics, true, true, 10);

        h_box.pack_start(&left_box, true, true, 0);
        let sep = gtk::Separator::new(gtk::Orientation::Vertical);
        h_box.pack_start(&sep, false, false, 0);
        h_box.pack_start(&right_box, false, false, 100);

        container.pack_start(&menu_bar, false, false, 0);
        container.pack_start(&h_box, true, true, 0);

        start_btn.connect_clicked({
            clone!(controller);
            move |btn| {
                btn.set_sensitive(false);
                ton_combo.set_sensitive(false);
                revealer.set_reveal_child(true);

                let pos = ton_combo.get_active() as usize;
                let tonality = octaves::note::TONALITIES.get(pos).unwrap();
                controller.borrow_mut().new_game(*tonality);
                controller.borrow().play_tonal_center();
            }
        });

        Content { container }
    }

    fn build_statistics_panel() -> gtk::Box {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let box_1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let box_2 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let box_3 = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let label_1 = gtk::Label::new("Right:");
        let label_2 = gtk::Label::new("X");

        let label_3 = gtk::Label::new("Total:");
        let label_4 = gtk::Label::new("Y");

        box_1.pack_start(&label_1, false, false, 0);
        box_1.pack_end(&label_2, false, false, 0);

        box_2.pack_start(&label_3, false, false, 0);
        box_2.pack_end(&label_4, false, false, 0);

        for i in 1..6 {
            let s = format!("{}. ...", i);
            let l = gtk::Label::new(s.as_str());
            box_3.pack_start(&l, false, false, 0);
        }

        container.pack_start(&box_1, false, false, 0);
        container.pack_start(&box_2, false, false, 0);
        container.pack_end(&box_3, false, false, 0);

        container
    }

    fn build_game_area(controller: &octaves::SharedController) -> gtk::Box {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        v_box.set_halign(gtk::Align::Center);
        v_box.set_valign(gtk::Align::Center);

        let box_1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        box_1.set_halign(gtk::Align::Center);

        let btns = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        btns.set_halign(gtk::Align::Center);

        let keys = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let play_chord_btn =
            gtk::Button::new_from_icon_name("emblem-music-symbolic", gtk::IconSize::Button.into());
        play_chord_btn.set_tooltip_text("Play chord");
        play_chord_btn.connect_clicked({
            clone!(controller);
            move |_| {
                controller.borrow().play_tonal_center();
            }
        });

        let play_btn = gtk::Button::new_from_icon_name(
            "media-playback-start-symbolic",
            gtk::IconSize::Button.into(),
        );
        play_btn.set_tooltip_text("Repeat note");
        play_btn.connect_clicked({
            clone!(controller);
            move |_| {
                controller.borrow().repeat_note();
            }
        });

        let next_btn =
            gtk::Button::new_from_icon_name("go-next-symbolic", gtk::IconSize::Button.into());
        next_btn.set_tooltip_text("Play next note");
        next_btn
            .get_style_context()
            .map(|x| x.add_class("suggested-action"));
        next_btn.connect_clicked({
            clone!(controller);
            move |_| {
                controller.borrow_mut().play_next_note();
            }
        });

        let c_btn = gtk::Button::new_with_label("C");
        let d_btn = gtk::Button::new_with_label("D");
        let e_btn = gtk::Button::new_with_label("E");
        let f_btn = gtk::Button::new_with_label("F");
        let g_btn = gtk::Button::new_with_label("G");
        let a_btn = gtk::Button::new_with_label("A");
        let b_btn = gtk::Button::new_with_label("B");

        let note_btns = [&c_btn, &d_btn, &e_btn, &f_btn, &g_btn, &a_btn, &b_btn];
        for btn in note_btns.iter() {
            btn.connect_clicked({
                clone!(controller);
                move |b| {
                    let label = b.get_label().unwrap();
                    controller.borrow_mut().check_answer(&label);
                }
            });
        }

        box_1.pack_start(&play_chord_btn, false, false, 0);

        btns.pack_start(&play_btn, false, false, 0);
        btns.pack_start(&next_btn, false, false, 0);

        keys.pack_start(&c_btn, false, false, 0);
        keys.pack_start(&d_btn, false, false, 0);
        keys.pack_start(&e_btn, false, false, 0);
        keys.pack_start(&f_btn, false, false, 0);
        keys.pack_start(&g_btn, false, false, 0);
        keys.pack_start(&a_btn, false, false, 0);
        keys.pack_start(&b_btn, false, false, 0);

        v_box.pack_start(&box_1, false, false, 0);
        v_box.pack_start(&btns, false, false, 20);
        v_box.pack_start(&keys, false, false, 0);

        v_box
    }
}
