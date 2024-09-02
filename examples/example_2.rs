use animaterm::prelude::*;
use animaterm::utilities::message_box;
use std::collections::HashMap;

fn main() {
    let mut mgr = Manager::new(true, None, None, None, None, None);

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
    let ordering = vec![
        (start_frame, Timestamp::new(0, 500)),
        (start_frame + 1, Timestamp::new(0, 500)),
    ];
    let running = false;
    let looping = true;
    let start_time = Timestamp::now();
    let animation = Animation::new(running, looping, ordering, start_time);
    let mut animations = HashMap::new();
    let anim_id = 0;
    animations.insert(anim_id, animation);
    let mut gr = Graphic::new(cols, rows, start_frame, library, Some(animations));
    let fast_ordering = vec![
        (start_frame, Timestamp::new(0, 200)),
        (start_frame + 1, Timestamp::new(0, 200)),
    ];
    let mut fast_anim_id = anim_id;
    if let Some(id) = gr.add_animation(Animation::new(running, looping, fast_ordering, start_time))
    {
        fast_anim_id = id;
    };

    let layer = 0;
    let offset = (15, 5);
    let graphic_id = mgr.add_graphic(gr, layer, offset).unwrap();
    let screen_size = mgr.screen_size();
    let title = "Navigation help".to_string();
    let text = "Press 0 to set current frame to 0\n Press 1 to set current frame to 1\n Press a|Shift+a|Ctrl+a start anim \n Press s or Shift+s stop animation\n\n Press q or Shift+q to quit\n".to_string();
    let mbox = message_box(Some(title), text, Glyph::default(), 80, 17);
    let mbid = mgr
        .add_graphic(mbox, 1, (1, screen_size.1 as isize - 8))
        .unwrap();
    mgr.set_graphic(mbid, 0, true);

    let var_ordering = vec![
        (start_frame, Timestamp::new(0, 400)),
        (start_frame + 1, Timestamp::new(0, 400)),
        (start_frame, Timestamp::new(0, 300)),
        (start_frame + 1, Timestamp::new(0, 300)),
        (start_frame, Timestamp::new(0, 200)),
        (start_frame + 1, Timestamp::new(0, 200)),
        (start_frame, Timestamp::new(0, 100)),
        (start_frame + 1, Timestamp::new(0, 100)),
        (start_frame, Timestamp::new(0, 200)),
        (start_frame + 1, Timestamp::new(0, 200)),
        (start_frame, Timestamp::new(0, 300)),
        (start_frame + 1, Timestamp::new(0, 300)),
    ];
    // let mut all_results_read = false;
    // while !all_results_read {
    //     let result = mgr.read_result();
    //     match result {
    //         Ok(AnimOk::AllResultsRead) => all_results_read = true,
    //         _ => continue,
    //     }
    // }
    mgr.add_animation(
        graphic_id,
        Animation::new(false, true, var_ordering, Timestamp::now()),
    );
    let mut var_anim_id = 0;

    if let Ok(AnimOk::AnimationAdded(anim_id)) = mgr.read_result() {
        var_anim_id = anim_id;
    }

    let mut keep_running = true;
    while keep_running {
        if let Some(key) = mgr.read_key() {
            match key {
                Key::Zero => mgr.set_graphic(graphic_id, start_frame, true),
                Key::One => mgr.set_graphic(graphic_id, start_frame + 1, true),
                Key::A => mgr.start_animation(graphic_id, anim_id),
                Key::ShiftA => mgr.start_animation(graphic_id, fast_anim_id),
                Key::P => mgr.pause_animation(graphic_id),
                Key::CtrlP => mgr.pause_animation(graphic_id),
                Key::ShiftP => mgr.pause_animation_on_frame(graphic_id, start_frame),
                Key::CtrlA => mgr.start_animation(graphic_id, var_anim_id),
                Key::S | Key::ShiftS => mgr.stop_animation(graphic_id),
                Key::Q | Key::ShiftQ => {
                    keep_running = false;
                }
                _ => continue,
            }
        }
    }

    mgr.terminate();
}
