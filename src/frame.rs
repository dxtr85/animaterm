use super::Glyph;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Read a frame from file.
pub fn from_file<P>(filename: &P) -> Option<(usize, Vec<Glyph>)>
where
    P: AsRef<Path>,
{
    let mut result = None;
    if let Ok(file) = File::open(filename) {
        let mut read_string = String::with_capacity(1024);
        let mut br = io::BufReader::new(file);
        if br.read_to_string(&mut read_string).is_ok() {
            let mut cs = 0;
            let mut rs = 0;
            let mut glyph = Glyph::default();
            let mut frame = Vec::new();
            for line in read_string.lines() {
                rs += 1;
                let mut style_started = false;
                let mut style_definition = String::new();
                for char in line.chars() {
                    match char {
                        '\x1b' => {
                            if style_definition.len() > 0 {
                                glyph.update_from_str(&style_definition);
                                style_definition.clear();
                            }
                            style_started = true;
                            style_definition.push(char);
                        }
                        'm' => {
                            style_definition.push(char);
                            if style_started {
                                style_started = false;
                            } else {
                                glyph.update_from_str(&style_definition);
                                frame.push(glyph);
                                style_definition.clear();
                                cs += 1;
                            }
                        }
                        '\n' => {
                            continue;
                        }
                        _ => {
                            style_definition.push(char);
                            if !style_started {
                                glyph.update_from_str(&style_definition);
                                frame.push(glyph);
                                style_definition.clear();
                                cs += 1;
                            }
                        }
                    }
                }
            }
            cs = cs / rs;
            if frame.len() > 0 {
                result = Some((cs, frame));
            } else {
                eprintln!("Frame empty!");
            }
        } else {
            eprintln!("Unable to read file!");
        }
    }
    result
}
