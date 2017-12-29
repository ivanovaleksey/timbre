extern crate timbre;

use timbre::games::octaves;

fn main() {
    let mut controller = octaves::Controller::new();

    let tonality = octaves::note::TONALITIES.first().unwrap();
    controller.new_game(*tonality);

    println!("{:?}", controller);

    controller.play_sequence();

    controller.play_note(None);
    controller.repeat_note();

    controller.play_note(None);
    controller.play_note(None);
    controller.repeat_note();

    controller.play_note(None);
    controller.play_note(None);
    controller.repeat_note();

    controller.play_note(None);
    controller.play_note(None);
    controller.repeat_note();

    controller.play_note(None);
    controller.play_note(None);
    controller.repeat_note();

    controller.play_note(None);
    controller.repeat_note();

    std::thread::sleep_ms(1000 * 60 * 2);
}
