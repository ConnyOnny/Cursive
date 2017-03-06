//! The terminal-optimized word wrapping utils from the Cursive project

#![deny(missing_docs)]
extern crate unicode_segmentation;
extern crate unicode_width;

pub mod utils;
mod with;
pub use with::With;

#[test]
fn wide_char_test() {
    let s = "Ｈｅｌｌo";
    let it = utils::LinesIterator::new(s,5);
    let vec : Vec<_> = it.map(|r| &s[r.start..r.end]).collect();
    // these are wide variants of H, e and l.
    let expected = vec!["Ｈｅ", "ｌｌo"];
    assert_eq!(vec, expected);
}

#[test]
fn prefix_exactly() {
    assert_eq!(utils::prefix("abra a".split(' '), 5, " ").length, 4);
}

#[test]
fn wrap_exactly() {
    let s = "abra a";
    println!("test wrapping");
    let it = utils::LinesIterator::new(s,5);
    let vec : Vec<_> = it.map(|r| &s[r.start..r.end]).collect();
    assert_eq!(vec, vec!["abra","a"]);
    println!("test NOT wrapping");
    let it = utils::LinesIterator::new(s,6);
    let vec : Vec<_> = it.map(|r| &s[r.start..r.end]).collect();
    assert_eq!(vec, vec!["abra a"]);
}
