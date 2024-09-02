use animaterm::prelude::*;
use animaterm::utilities::{message_box, progress_bar};

fn main() {
    let mut mgr = Manager::new(true, None, None, None, None, None);

    let screen_size = mgr.screen_size();
    let title = "Navigation help".to_string();
    let text =
        "Press 0 to select display 0\n Press 1 to select display 1\n Press q or Shift+q to quit\n"
            .to_string();
    let keep_existing = true;
    let first_display_id = 0;
    let mbox = message_box(Some(title.clone()), text.clone(), Glyph::default(), 32, 5);
    let mbid = mgr
        .add_graphic(mbox, 1, (1, screen_size.1 as isize - 6))
        .unwrap();
    mgr.set_graphic(mbid, 0, true);
    let empty = Glyph::new(
        ' ',
        Color::green(),
        Color::black(),
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    let full = Glyph::new(
        'X',
        Color::green(),
        Color::black(),
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    let gid = mgr
        .add_graphic(progress_bar(screen_size.0, empty, full, None), 0, (1, 1))
        .unwrap();
    mgr.start_animation(gid, 0);
    let second_display_id = mgr.new_display(keep_existing);
    // let result = mgr.read_result();
    // if let Ok(AnimOk::DisplayStored(disp_id)) = result {
    //     first_display_id = disp_id;
    // }
    let mbox = message_box(Some(title), text, Glyph::default(), 32, 5);
    let mbid = mgr
        .add_graphic(mbox, 1, (1, screen_size.1 as isize - 6))
        .unwrap();
    mgr.set_graphic(mbid, 0, true);

    let mut keep_running = true;
    while keep_running {
        if let Some(key) = mgr.read_key() {
            match key {
                Key::Zero => mgr.restore_display(first_display_id, true),
                Key::One => mgr.restore_display(second_display_id, true),
                Key::Two => mgr.restore_display(2, true),
                Key::Q | Key::ShiftQ => {
                    keep_running = false;
                }
                _ => continue,
            }
        }
    }

    mgr.terminate();
}
