//! ` `
//! The space factor does not change when we append a discretionary node,
//! but it starts out as 1000 in the subsidiary lists.
//
// @<Declare act...@>=
// procedure append_discretionary;
pub(crate) fn append_discretionary(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var c:integer; {hyphen character}
    // begin tail_append(new_disc);
    tail_append!(globals, new_disc(globals)?);
    // if cur_chr=1 then
    if globals.cur_chr.get() == 1 {
        /// hyphen character
        let c;
        // begin c:=hyphen_char[cur_font];
        c = globals.hyphen_char[cur_font!(globals)];
        // if c>=0 then if c<256 then pre_break(tail):=new_character(cur_font,c);
        if c >= 0 && c <= ASCII_code::max_allowed_repr(globals) {
            pre_break!(globals, tail!(globals)) =
                new_character(globals, cur_font!(globals).into(), ASCII_code::from(c));
        }
        // end
    }
    // else  begin incr(save_ptr); saved(-1):=0; new_save_level(disc_group);
    else {
        incr!(globals.save_ptr);
        saved!(globals, @neg 1) = 0;
        new_save_level(globals, disc_group.into());
        // scan_left_brace; push_nest; mode:=-hmode; space_factor:=1000;
        scan_left_brace(globals)?;
        push_nest(globals);
        mode!(globals) = (-hmode).into();
        space_factor!(globals) = 1000;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0145::new_disc;
use crate::section_0145::pre_break;
use crate::section_0211::hmode;
use crate::section_0213::mode;
use crate::section_0213::space_factor;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0216::push_nest;
use crate::section_0230::cur_font;
use crate::section_0269::disc_group;
use crate::section_0274::new_save_level;
use crate::section_0274::saved;
use crate::section_0403::scan_left_brace;
use crate::section_0582::new_character;
