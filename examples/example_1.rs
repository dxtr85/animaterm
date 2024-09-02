use animaterm::prelude::*;
use animaterm::utilities::message_box;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let macros = Some(vec![
        (Key::AltM, MacroSequence::empty()),
        (
            Key::T,
            MacroSequence::from_text(
                "This text was typed with a single key press!".to_string(),
                Duration::from_millis(100),
                false,
            ),
        ),
    ]);
    let mut mgr = Manager::new(true, None, None, None, None, macros);

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
    let text = "Press 0 to set current frame to 0\n Press 1 to set current frame to 1\n Press t to type text using macro\n Press AltM to define a macro\n\nPress q or Shift+q to quit\n".to_string();
    let mbox = message_box(Some(title), text, Glyph::default(), 37, 7);
    let mbid = mgr.add_graphic(mbox, 1, (1, screen_size.1 as isize - 10));
    if let Some(mid) = mbid {
        mgr.set_graphic(mid, 0, true);
    }

    let mut keep_running = true;
    mgr.move_cursor(1,1 );
    let mut macro_mode:u8 = 0;
    let mut looped = false;
    while keep_running {
            let key = mgr.read_key();
        if let Some(ch) = map_key_to_char(&key){
            if macro_mode ==0{
                print!("{}",ch);
            }
        }
        match key {
            Key::Zero => mgr.set_graphic(graphic_id, start_frame, true),
            Key::One => mgr.set_graphic(graphic_id, start_frame + 1, true),
            Key::AltM=>
                match macro_mode {
                    0 => {
                        // let (max_x, may_y)=mgr.screen_size();
                        // mgr.clear_area(0, (0,1),(max_x,4) );
                        println!("Press trigger key (or AltM again to toggle macro looping)");
                        macro_mode = 1;
                    },
                    1 =>{
                        looped = !looped;
                        println!("Macro looping: {}",looped);
                    },
                    2=>{
                        println!("Macro is defined!");
                        macro_mode = 0;
                        looped = false;
                    }
                    _=>{
                        println!("This should not happen");
                    }
                },
            Key::Q | Key::ShiftQ => {
                keep_running = false;
            },
            other => {
                if macro_mode == 1 {
                    println!("Macro trigger: {}",other);
                    println!("Not type macro sequence, followed by AltM");
                    macro_mode = 2;
                }else if macro_mode == 2{
                    println!("Macro sequence add: {}",other)
                }else{
                    continue
                }
            }
        }
    }

    mgr.terminate();
}
