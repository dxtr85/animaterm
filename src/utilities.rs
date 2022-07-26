use super::animation::Animation;
use super::glyph::Glyph;
use super::graphic::Graphic;
use super::pixel::Pixel;
use super::time::Timestamp;
use std::collections::HashMap;
use std::mem::replace;

pub fn progress_bar(
    width: usize,
    empty: Glyph,
    full: Glyph,
    stages: Option<Vec<Glyph>>,
) -> Graphic {
    let mut total_states = 1;
    let mut states = vec![]; //empty.clone()];
    if let Some(stages) = stages {
        total_states += stages.len();
        for s in stages {
            states.push(s)
        }
    }
    states.push(full.clone());
    let mut library = HashMap::with_capacity(width * total_states);
    let mut ordering = Vec::with_capacity(total_states);
    let mut contstruction_state = vec![empty; width];
    let mut j = 0;
    for i in 0..width {
        for state_no in 0..total_states {
            replace(
                &mut contstruction_state[i],
                states.get(state_no).unwrap().clone(),
            );
            library.insert(j, contstruction_state.clone());
            j += 1;
            ordering.push((j % (total_states * width), Timestamp::new(0, 1)));
        }
    }
    let mut anim = Animation::new(HashMap::new(), true, true, ordering, Timestamp::now());
    let mut anims = HashMap::new();
    anims.insert(0, anim);
    Graphic::new(width, 1, 0, library, Some(anims))
}

pub fn message_box(
    title: Option<String>,
    content: String,
    glyph: Glyph,
    start_x: usize,
    start_y: usize,
    width: usize,
    lenght: usize,
) -> Vec<Pixel> {
    let mut mbox = Vec::new();
    let mut cgl = glyph.clone();
    cgl.set_char('╭');
    mbox.push(Pixel::new(start_x, start_y, true, cgl.clone()));
    let mut i = 1;
    if let Some(name) = title {
        for c in name.chars() {
            if i > width.saturating_sub(2) {
                break;
            }
            cgl.set_char(c);
            mbox.push(Pixel::new(start_x + i, start_y, true, cgl.clone()));
            i += 1;
        }
    }
    cgl.set_char('─');
    for i in start_x + i..start_x + width - 1 {
        mbox.push(Pixel::new(i, start_y, true, cgl.clone()));
    }
    cgl.set_char('╮');
    mbox.push(Pixel::new(start_x + width - 1, start_y, true, cgl.clone()));
    let mut text = content.split_whitespace();
    let mut word = text.next();
    for j in start_y + 1..start_y + lenght - 1 {
        cgl.set_char('│');
        mbox.push(Pixel::new(start_x, j, true, cgl.clone()));
        i = 2;
        mbox.push(Pixel::new(start_x + 1, j, true, glyph.clone()));
        if let Some(mut content) = word {
            while content.len() < width.saturating_sub(i + 1) {
                for c in content.chars() {
                    cgl.set_char(c);
                    mbox.push(Pixel::new(start_x + i, j, true, cgl.clone()));
                    i += 1;
                }
                mbox.push(Pixel::new(start_x + i, j, true, glyph.clone()));
                i += 1;
                word = text.next();
                if let Some(help) = word {
                    content = help;
                } else {
                    content = "";
                }
            }
            for g in start_x + i..start_x + width - 1 {
                mbox.push(Pixel::new(g, j, true, glyph.clone()));
            }
        } else {
            for i in start_x + 1..start_x + width - 1 {
                mbox.push(Pixel::new(i, j, true, glyph.clone()));
            }
        }
        cgl.set_char('│');
        mbox.push(Pixel::new(start_x + width - 1, j, true, cgl.clone()));
    }
    cgl.set_char('╰');
    mbox.push(Pixel::new(start_x, start_y + lenght - 1, true, cgl.clone()));
    cgl.set_char('─');
    for i in start_x + 1..start_x + width - 1 {
        mbox.push(Pixel::new(i, start_y + lenght - 1, true, cgl.clone()));
    }
    cgl.set_char('╯');
    mbox.push(Pixel::new(
        start_x + width - 1,
        start_y + lenght - 1,
        true,
        cgl.clone(),
    ));
    mbox
}

pub fn new_message_box(
    title: Option<String>,
    content: String,
    glyph: Glyph,
    width: usize,
    height: usize,
) -> Graphic {
    let mut mbox = Vec::with_capacity(width * height);
    let mut cgl = glyph.clone();
    cgl.set_char('╭');
    mbox.push(cgl.clone());
    let mut i = 1;
    if let Some(name) = title {
        for c in name.chars() {
            if i > width.saturating_sub(2) {
                break;
            }
            cgl.set_char(c);
            mbox.push(cgl.clone());
            i += 1;
        }
    }
    cgl.set_char('─');
    for _i in i..width - 1 {
        mbox.push(cgl.clone());
    }
    cgl.set_char('╮');
    mbox.push(cgl.clone());

    let mut text = content.split_whitespace();
    let mut word = text.next();
    for _j in 1..height - 1 {
        cgl.set_char('│');
        mbox.push(cgl.clone());
        i = 2;
        mbox.push(glyph.clone());
        if let Some(mut content) = word {
            while content.len() < width.saturating_sub(i + 1) {
                for c in content.chars() {
                    cgl.set_char(c);
                    mbox.push(cgl.clone());
                    i += 1;
                }
                mbox.push(glyph.clone());
                i += 1;
                word = text.next();
                if let Some(help) = word {
                    content = help;
                } else {
                    content = "";
                }
            }
            for _g in i..width - 1 {
                mbox.push(glyph.clone());
            }
        } else {
            for _i in 1..width - 2 {
                mbox.push(glyph.clone());
            }
        }
        cgl.set_char('│');
        mbox.push(cgl.clone());
    }
    cgl.set_char('╰');
    mbox.push(cgl.clone());
    cgl.set_char('─');
    for _i in 1..width - 1 {
        mbox.push(cgl.clone());
    }
    cgl.set_char('╯');
    mbox.push(cgl.clone());

    let mut library = HashMap::new();
    library.insert(0, mbox);
    Graphic::new(width, height, 0, library, None)
}
