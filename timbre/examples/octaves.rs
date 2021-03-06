extern crate timbre;

use timbre::games::octaves;

fn main() {
    let config = octaves::Config::load();
    let config_1 = config.clone();
    let mut controller = octaves::Controller::new(config);

    let tonality = octaves::note::TONALITIES.first().unwrap();
    controller.new_game(*tonality);

    controller.play_tonal_center();

    controller.play_next_note();
    controller.repeat_note();

    controller.play_next_note();
    controller.play_next_note();
    controller.repeat_note();

    controller.play_next_note();
    controller.play_next_note();
    controller.repeat_note();

    controller.play_next_note();
    controller.play_next_note();
    controller.repeat_note();

    controller.play_next_note();
    controller.play_next_note();
    controller.repeat_note();

    controller.play_next_note();
    controller.repeat_note();

    config_1.save();
    std::thread::sleep_ms(1000 * 60 * 2);
}
