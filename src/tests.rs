#![cfg(test)]
use super::glyphcake::GlyphCake;
use super::Glyph;
use super::Timestamp;
#[test]
fn overflow_ms_to_sec() {
    let t0 = Timestamp::new(0, 0);
    let t1 = Timestamp::new(0, 1000);
    assert_eq!(t0 + t1, Timestamp::new(1, 0));
}
#[test]
fn add_sec_to_sec() {
    let t0 = Timestamp::new(1, 500);
    let t1 = Timestamp::new(2, 700);
    assert_eq!(t0 + t1, Timestamp::new(4, 200));
}
#[test]
fn sub_sec_from_sec() {
    let t0 = Timestamp::new(1, 500);
    let t1 = Timestamp::new(1, 400);
    assert_eq!(t0 - t1, Timestamp::new(0, 100));
}
#[test]
fn no_neg_time() {
    let t0 = Timestamp::new(1, 500);
    let t1 = Timestamp::new(2, 400);
    assert_eq!(t0 - t1, Timestamp::new(0, 0));
}
#[test]
fn glyph_cake() {
    let g1 = Glyph::new(
        '1',
        crate::Color::white(),
        crate::Color::red(),
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    let g2 = Glyph::new(
        '2',
        crate::Color::blue(),
        crate::Color::yellow(),
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
    );
    let mut gc = GlyphCake::new(1, 1, Some(g1), 2);
    //println!("Start: {:?}", gc.glyphs);
    gc.update(g2, 10);
    //println!("Update1: {:?}", gc.glyphs);
    assert_eq!(gc.get_glyph().character, '2');
    println!("Update2before: {:?}", gc.glyphs);
    gc.update(Glyph::transparent(), 10);
    println!("Update2: {:?}", gc.glyphs);
    assert_eq!(gc.glyphs.len(), 3);
    assert_eq!(gc.get_glyph().character, '1');
}
#[test]
fn update_glyph_from_str() {
    let mut g = Glyph::default();
    g.update_from_str("\x1b[2;23;24;25;26;27;29;30;48;5;196m ");
    assert_eq!(g.background, crate::Color::EightBit(196));
}
#[test]
fn update_glyph_twice_from_str() {
    let mut g = Glyph::default();
    g.update_from_str("\x1b[48;2;91;63;43m");
    g.update_from_str("\x1b[38;2;90;62;42m▀");
    assert_eq!(g.background, crate::Color::Truecolor(91, 63, 43));
    assert_eq!(g.color, crate::Color::Truecolor(90, 62, 42));
}
#[test]
fn glyph_cake2() {
    let mut insert_glyph = Glyph::default();
    let mut gc = GlyphCake::new(0, 0, Some(insert_glyph), 0);
    insert_glyph.set_background(crate::Color::blue());
    gc.update(insert_glyph, 1);
    let g = gc.get_glyph();
    assert_eq!(g.background, crate::Color::blue());
    gc.update(Glyph::transparent(), 1);
    let g = gc.get_glyph();
    assert_eq!(g.background, crate::Color::black());
    assert_eq!(g.color, crate::Color::white());
}
