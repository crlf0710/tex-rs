//! @ First let's consider the format of whatsit nodes that are used to represent
//! the data associated with \.{\\write} and its relatives. Recall that a whatsit
//! has |type=whatsit_node|, and the |subtype| is supposed to distinguish
//! different kinds of whatsits. Each node occupies two or more words; the
//! exact number is immaterial, as long as it is readily determined from the
//! |subtype| or other data.
//!
//! We shall introduce five |subtype| values here, corresponding to the
//! control sequences \.{\\openout}, \.{\\write}, \.{\\closeout}, \.{\\special}, and
//! \.{\\setlanguage}. The second word of I/O whatsits has a |write_stream| field
//! that identifies the write-stream number (0 to 15, or 16 for out-of-range and
//! positive, or 17 for out-of-range and negative).
//! In the case of \.{\\write} and \.{\\special}, there is also a field that
//! points to the reference count of a token list that should be sent. In the
//! case of \.{\\openout}, we need three words and three auxiliary subfields
//! to hold the string numbers for name, area, and extension.
//
// @d write_node_size=2 {number of words in a write/whatsit node}
/// number of words in a write/whatsit node
pub(crate) const write_node_size: quarterword = 2;
// @d open_node_size=3 {number of words in an open/whatsit node}
/// number of words in an open/whatsit node
pub(crate) const open_node_size: quarterword = 3;
// @d open_node=0 {|subtype| in whatsits that represent files to \.{\\openout}}
/// `subtype` in whatsits that represent files to `\openout`
pub(crate) const open_node: quarterword = 0;
// @d write_node=1 {|subtype| in whatsits that represent things to \.{\\write}}
/// `subtype` in whatsits that represent things to `\write`
pub(crate) const write_node: quarterword = 1;
// @d close_node=2 {|subtype| in whatsits that represent streams to \.{\\closeout}}
/// `subtype` in whatsits that represent streams to `\closeout`
pub(crate) const close_node: quarterword = 2;
// @d special_node=3 {|subtype| in whatsits that represent \.{\\special} things}
/// `subtype` in whatsits that represent `\special` things
pub(crate) const special_node: quarterword = 3;
// @d language_node=4 {|subtype| in whatsits that change the current language}
/// `subtype` in whatsits that change the current language
pub(crate) const language_node: quarterword = 4;
// @d what_lang(#)==link(#+1) {language number, in the range |0..255|}
// @d what_lhm(#)==type(#+1) {minimum left fragment, in the range |1..63|}
// @d what_rhm(#)==subtype(#+1) {minimum right fragment, in the range |1..63|}
// @d write_tokens(#) == link(#+1) {reference count of token list to write}
/// reference count of token list to write
macro_rules! write_tokens {
    ($globals:expr, $ptr:expr) => {
        link!($globals, $ptr + 1)
    }
}
// @d write_stream(#) == info(#+1) {stream number (0 to 17)}
/// stream number (0 to 17)
macro_rules! write_stream {
    ($globals:expr, $ptr:expr) => {
        info_inner!($globals, $ptr + 1)
    }
}
// @d open_name(#) == link(#+1) {string number of file name to open}
/// string number of file name to open
macro_rules! open_name {
    ($globals:expr, $ptr:expr) => {
        link!($globals, $ptr + 1)
    }
}
// @d open_area(#) == info(#+2) {string number of file area for |open_name|}
/// string number of file area for `open_name`
macro_rules! open_area {
    ($globals:expr, $ptr:expr) => {
        info_inner!($globals, $ptr + 2)
    }
}
// @d open_ext(#) == link(#+2) {string number of file extension for |open_name|}
/// string number of file extension for `open_name`
macro_rules! open_ext {
    ($globals:expr, $ptr:expr) => {
        link!($globals, $ptr + 2)
    }
}

use crate::section_0113::quarterword;