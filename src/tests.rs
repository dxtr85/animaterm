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
        crate::NewColor::white(),
        crate::NewColor::red(),
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
        crate::NewColor::blue(),
        crate::NewColor::yellow(),
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
    gc.update(Glyph::plain(), 10);
    println!("Update2: {:?}", gc.glyphs);
    assert_eq!(gc.glyphs.len(), 3);
    assert_eq!(gc.get_glyph().character, '1');
}
