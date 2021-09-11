//! @ Users refer to `\.{\\the\\spacefactor}' only in horizontal
//! mode, and to `\.{\\the\\prevdepth}' only in vertical mode; so we put the
//! associated mode in the modifier part of the |set_aux| command.
//! The |set_page_int| command has modifier 0 or 1, for `\.{\\deadcycles}' and
//! `\.{\\insertpenalties}', respectively. The |set_box_dimen| command is
//! modified by either |width_offset|, |height_offset|, or |depth_offset|.
//! And the |last_item| command is modified by either |int_val|, |dimen_val|,
//! |glue_val|, |input_line_no_code|, or |badness_code|.
//
// @d input_line_no_code=glue_val+1 {code for \.{\\inputlineno}}
// @d badness_code=glue_val+2 {code for \.{\\badness}}
#[doc(hidden)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub(crate) enum last_item_command_kind {
    /// integer values
    int_val = 0,
    /// dimension values
    dimen_val = 1,
    /// glue specifications
    glue_val = 2,
    /// code for `\inputlineno`
    input_line_no_code,
    /// code for `\badness`
    badness_code,
}

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0416(globals: &mut TeXGlobals) {
    // primitive("spacefactor",set_aux,hmode);
    // @!@:space_factor_}{\.{\\spacefactor} primitive@>
    // primitive("prevdepth",set_aux,vmode);@/
    primitive(
        globals,
        crate::strpool_str!("prevdepth"),
        set_aux,
        vmode as _,
    );
    // @!@:prev_depth_}{\.{\\prevdepth} primitive@>
    // primitive("deadcycles",set_page_int,0);
    // @!@:dead_cycles_}{\.{\\deadcycles} primitive@>
    // primitive("insertpenalties",set_page_int,1);
    primitive(
        globals,
        crate::strpool_str!("insertpenalties"),
        set_page_int,
        1,
    );
    // @!@:insert_penalties_}{\.{\\insertpenalties} primitive@>
    // primitive("wd",set_box_dimen,width_offset);
    primitive(
        globals,
        crate::strpool_str!("wd"),
        set_box_dimen,
        width_offset as _,
    );
    // @!@:wd_}{\.{\\wd} primitive@>
    // primitive("ht",set_box_dimen,height_offset);
    // @!@:ht_}{\.{\\ht} primitive@>
    // primitive("dp",set_box_dimen,depth_offset);
    primitive(
        globals,
        crate::strpool_str!("dp"),
        set_box_dimen,
        depth_offset as _,
    );
    // @!@:dp_}{\.{\\dp} primitive@>
    // primitive("lastpenalty",last_item,int_val);
    // @!@:last_penalty_}{\.{\\lastpenalty} primitive@>
    // primitive("lastkern",last_item,dimen_val);
    // @!@:last_kern_}{\.{\\lastkern} primitive@>
    // primitive("lastskip",last_item,glue_val);
    // @!@:last_skip_}{\.{\\lastskip} primitive@>
    // primitive("inputlineno",last_item,input_line_no_code);
    primitive(
        globals,
        crate::strpool_str!("inputlineno"),
        last_item,
        last_item_command_kind::input_line_no_code as _,
    );
    // @!@:input_line_no_}{\.{\\inputlineno} primitive@>
    // primitive("badness",last_item,badness_code);
    primitive(
        globals,
        crate::strpool_str!("badness"),
        last_item,
        last_item_command_kind::badness_code as _,
    );
    // @!@:badness_}{\.{\\badness} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0135::depth_offset;
use crate::section_0135::width_offset;
use crate::section_0208::last_item;
use crate::section_0209::set_aux;
use crate::section_0209::set_box_dimen;
use crate::section_0209::set_page_int;
use crate::section_0211::vmode;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
