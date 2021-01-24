//! @ The \.{\\language} extension is somewhat different.
//! We need a subroutine that comes into play when a character of
//! a non-|clang| language is being appended to the current paragraph.
//
// @<Declare action...@>=
// procedure fix_language;
pub(crate) fn fix_language(globals: &mut TeXGlobals) -> TeXResult<()> {
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
        // begin new_whatsit(language_node,small_node_size);
        new_whatsit(globals, language_node.into(), small_node_size.into())?;
        // what_lang(tail):=l; clang:=l;@/
        what_lang!(globals, tail!(globals)) = l.numeric_value() as _;
        clang!(globals) = l.numeric_value() as _;
        // what_lhm(tail):=norm_min(left_hyphen_min);
        what_lhm!(globals, tail!(globals)) = norm_min(left_hyphen_min!(globals)).get();
        // what_rhm(tail):=norm_min(right_hyphen_min);
        what_rhm!(globals, tail!(globals)) = norm_min(right_hyphen_min!(globals)).get();
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0141::small_node_size;
use crate::section_1091::norm_min;
use crate::section_1341::language_node;
use crate::section_1349::new_whatsit;
