use gtk;
use gtk::prelude::*;

use timbre::games::octaves;

#[derive(Clone)]
pub struct Content {
    pub container: gtk::Box,
    pub revealer: gtk::Revealer,
    pub start_btn: gtk::Button,
    pub tonality_combo: gtk::ComboBoxText,
}

struct Statistics {
    container: gtk::Box,
    right_label: gtk::Label,
    total_label: gtk::Label,
}

impl Content {
    pub fn new(controller: &octaves::SharedController) -> Content {
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let ton_combo = gtk::ComboBoxText::new();
        for tonality in octaves::note::TONALITIES.iter() {
            ton_combo.append_text(&format!("{:?}", tonality));
        }
        ton_combo.set_active(0);

        let start_btn = gtk::Button::new_with_label("Start");

        let stats = Content::build_statistics_panel();

        let game_area = Content::build_game_area(controller, &stats);
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
        right_box.pack_start(&stats.container, true, true, 10);

        container.pack_start(&left_box, true, true, 10);
        let sep = gtk::Separator::new(gtk::Orientation::Vertical);
        container.pack_start(&sep, false, false, 0);
        container.pack_start(&right_box, false, false, 100);

        start_btn.connect_clicked({
            clone!(controller, revealer, ton_combo);
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

        Content {
            container,
            start_btn,
            revealer,
            tonality_combo: ton_combo,
        }
    }

    fn build_statistics_panel() -> Statistics {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let box_1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let box_2 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let box_3 = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let label_1 = gtk::Label::new("Right: ");
        let label_2 = gtk::Label::new("0");

        let label_3 = gtk::Label::new("Total: ");
        let label_4 = gtk::Label::new("0");

        box_1.pack_start(&label_1, false, false, 0);
        box_1.pack_end(&label_2, false, false, 0);

        box_2.pack_start(&label_3, false, false, 0);
        box_2.pack_end(&label_4, false, false, 0);

        use diesel::prelude::*;
        use timbre;
        use timbre::schema::{octave_game_states, octave_games};
        use timbre::games::octaves::models;

        let conn = timbre::establish_connection();
        let history = octave_games::table
            .inner_join(octave_game_states::table)
            .select(octave_game_states::all_columns)
            .filter(octave_games::finished_at.is_not_null())
            .order(octave_games::created_at.desc())
            .limit(5)
            .load::<models::GameState>(&conn)
            .unwrap();

        for (i, game) in history.iter().enumerate() {
            let s = format!(
                "{}. ex. #{}, {} / {}",
                i + 1,
                game.exercise,
                game.right_count,
                game.total_count,
            );
            let l = gtk::Label::new(s.as_str());
            box_3.pack_start(&l, false, false, 0);
        }

        container.pack_start(&box_1, false, false, 0);
        container.pack_start(&box_2, false, false, 0);
        container.pack_end(&box_3, false, false, 0);

        Statistics {
            container,
            right_label: label_2,
            total_label: label_4,
        }
    }

    fn build_game_area(controller: &octaves::SharedController, stats: &Statistics) -> gtk::Box {
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        v_box.set_halign(gtk::Align::Center);
        v_box.set_valign(gtk::Align::Center);

        let box_1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        box_1.set_halign(gtk::Align::Center);

        let btns = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        btns.set_halign(gtk::Align::Center);

        let keys = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let play_chord_btn =
            gtk::Button::new_from_icon_name("emblem-music-symbolic", gtk::IconSize::Button.into());
        play_chord_btn.set_tooltip_text("Play tonal center");
        play_chord_btn.connect_clicked({
            clone!(controller);
            move |_| {
                controller.borrow().play_tonal_center();
            }
        });

        let play_btn = gtk::Button::new_from_icon_name(
            "media-playlist-repeat-symbolic",
            gtk::IconSize::Button.into(),
        );
        play_btn.set_tooltip_text("Repeat note");
        play_btn.connect_clicked({
            clone!(controller);
            move |_| {
                controller.borrow().repeat_note();
            }
        });

        let next_btn = gtk::Button::new_from_icon_name(
            "media-playback-start-symbolic",
            gtk::IconSize::Button.into(),
        );
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

        macro_rules! answer {
            ($btn:ident) => {{
                let label = $btn.get_label().unwrap();
                $btn.connect_clicked({
                    let btn = $btn.clone();
                    clone!(controller);
                    move |_| {
                        let answers: [&str; 1] = [&label];
                        if let Some(res) = controller.borrow_mut().check_answers(&answers) {
                            toggle_btn_class(&btn, res);
                        }
                    }
                });
            }}
        }

        answer!(c_btn);
        answer!(d_btn);
        answer!(e_btn);
        answer!(f_btn);
        answer!(g_btn);
        answer!(a_btn);
        answer!(b_btn);

        let csharp_btn = gtk::Button::new_with_label("# / b");
        let dsharp_btn = gtk::Button::new_with_label("# / b");
        let fsharp_btn = gtk::Button::new_with_label("# / b");
        let gsharp_btn = gtk::Button::new_with_label("# / b");
        let asharp_btn = gtk::Button::new_with_label("# / b");

        macro_rules! alt_answer {
            ($btn:ident, $sharp:expr, $flat:expr) => {{
                $btn.connect_clicked({
                    let btn = $btn.clone();
                    clone!(controller);
                    move |_| {
                        let answers: [&str; 2] = [&$sharp, &$flat];
                        if let Some(res) = controller.borrow_mut().check_answers(&answers) {
                            toggle_btn_class(&btn, res);
                        }
                    }
                });
            }}
        }

        alt_answer!(csharp_btn, "Csharp", "Dflat");
        alt_answer!(dsharp_btn, "Dsharp", "Eflat");
        alt_answer!(fsharp_btn, "Fsharp", "Gflat");
        alt_answer!(gsharp_btn, "Gsharp", "Aflat");
        alt_answer!(asharp_btn, "Asharp", "Bflat");

        controller.borrow_mut().add_count_observer({
            let label = stats.total_label.clone();
            move |ctrl| {
                let count = ctrl.total_count();
                label.set_text(&count.to_string());
            }
        });
        controller.borrow_mut().add_count_observer({
            let label = stats.right_label.clone();
            move |ctrl| {
                let count = ctrl.right_count();
                label.set_text(&count.to_string());
            }
        });

        let row_1 = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let row_2 = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let left_part = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let right_part = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        row_1.pack_start(&left_part, false, false, 20);
        row_1.pack_start(&right_part, false, false, 5);

        left_part.pack_start(&csharp_btn, false, false, 0);
        left_part.pack_start(&dsharp_btn, false, false, 0);

        right_part.pack_start(&fsharp_btn, false, false, 0);
        right_part.pack_start(&gsharp_btn, false, false, 0);
        right_part.pack_start(&asharp_btn, false, false, 0);

        row_2.pack_start(&c_btn, false, false, 0);
        row_2.pack_start(&d_btn, false, false, 0);
        row_2.pack_start(&e_btn, false, false, 0);
        row_2.pack_start(&f_btn, false, false, 0);
        row_2.pack_start(&g_btn, false, false, 0);
        row_2.pack_start(&a_btn, false, false, 0);
        row_2.pack_start(&b_btn, false, false, 0);

        box_1.pack_start(&play_chord_btn, false, false, 0);

        btns.pack_start(&play_btn, false, false, 0);
        btns.pack_start(&next_btn, false, false, 0);

        keys.pack_start(&row_1, false, false, 0);
        keys.pack_start(&row_2, false, false, 0);

        v_box.pack_start(&box_1, false, false, 0);
        v_box.pack_start(&btns, false, false, 20);
        v_box.pack_start(&keys, false, false, 0);

        v_box
    }
}

fn toggle_btn_class(btn: &gtk::Button, flag: bool) {
    let btn_class = if flag {
        "suggested-action"
    } else {
        "destructive-action"
    };

    btn.get_style_context().map(|c| c.add_class(&btn_class));

    gtk::timeout_add_seconds(1, {
        clone!(btn);
        move || {
            btn.get_style_context().map(|c| c.remove_class(&btn_class));
            gtk::Continue(true)
        }
    });
}
