//! The terminal-optimized word wrapping utils from the Cursive project

#![deny(missing_docs)]
extern crate unicode_segmentation;
extern crate unicode_width;

pub mod utils;
mod with;
pub use with::With;

#[test]
fn simple_test() {
    let s = "Ｈｅｌｌo";
    let it = utils::LinesIterator::new(s,5);
    let vec : Vec<_> = it.map(|r| &s[r.start..r.end]).collect();
    // these are wide variants of H, e and l.
    let expected = vec!["Ｈｅ", "ｌｌo"];
    assert_eq!(vec, expected);
}
