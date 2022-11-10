use animaterm::prelude::*;
use animaterm::utilities::message_box;
use std::collections::HashMap;

fn main() {
    let mut mgr = Manager::new(true, None, None, None, None);

    let mut library = HashMap::with_capacity(2);
    let cols = 10;
    let rows = 5;
    let start_frame = 0;
    let glyph_1 = Glyph::new(
        '\u{2580}',
        Color::new_8bit(0, 0, 5),
        Color::new_8bit(5, 5, 0),
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    let glyph_2 = Glyph::new(
        '\u{258C}',
        Color::new_truecolor(255, 255, 255),
        Color::new_truecolor(255, 0, 0),
        false,
        true,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );

    library.insert(start_frame, vec![glyph_1; rows * cols]);
    library.insert(start_frame + 1, vec![glyph_2; rows * cols]);
    let gr = Graphic::new(cols, rows, start_frame, library, None);

    let layer = 0;
    let offset = (15, 5);
    let graphic_id = mgr.add_graphic(gr, layer, offset).unwrap();
    let screen_size = mgr.screen_size();
    let title = "Navigation help".to_string();
    let text = "Use arrows to move graphic around \nPress q or Shift+q to quit\n".to_string();
    let mbox = message_box(Some(title), text, Glyph::default(), 37, 5);
    let mbid = mgr
        .add_graphic(mbox, 1, (1, screen_size.1 as isize - 6))
        .unwrap();
    mgr.set_graphic(graphic_id, start_frame, true);
    mgr.set_graphic(mbid, 0, true);

    let mut keep_running = true;
    while keep_running {
        if let Some(key) = mgr.read_key() {
            match key {
                Key::Left => mgr.move_graphic(graphic_id, layer, (-1, 0)),
                Key::Right => mgr.move_graphic(graphic_id, layer, (1, 0)),
                Key::Up => mgr.move_graphic(graphic_id, layer, (0, -1)),
                Key::Down => mgr.move_graphic(graphic_id, layer, (0, 1)),
                Key::Q | Key::ShiftQ => {
                    keep_running = false;
                }
                _ => continue,
            }
        }
    }

    mgr.terminate();
}
