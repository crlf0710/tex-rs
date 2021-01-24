//! @ The \.{\\language} extension is somewhat different.
//! We need a subroutine that comes into play when a character of
//! a non-|clang| language is being appended to the current paragraph.
//
// @<Declare action...@>=
// procedure fix_language;
pub(crate) fn fix_language(globals: &mut TeXGlobals) {
    // var @!l:ASCII_code; {the new current language}
    /// the new current language
    let l: ASCII_code;
    // begin if language<=0 then l:=0
    if language!(globals) <= 0 {
        l = 0.into();
    }
    // else if language>255 then l:=0
    else if language!(globals) > 255 {
        l = 0.into();
    }
    // else l:=language;
    else {
        l = (language!(globals) as integer).into();
    }
    // if l<>clang then
    if l.numeric_value() as integer != clang!(globals) as integer {
        todo!("fix language");
        // begin new_whatsit(language_node,small_node_size);
        // what_lang(tail):=l; clang:=l;@/
        // what_lhm(tail):=norm_min(left_hyphen_min);
        // what_rhm(tail):=norm_min(right_hyphen_min);
        // end;
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;