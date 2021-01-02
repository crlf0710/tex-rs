//! @ The final region of |eqtb| contains the dimension parameters defined
//! here, and the 256 \.{\\dimen} registers.
//
// @d par_indent_code=0 {indentation of paragraphs}
/// indentation of paragraphs
pub(crate) const par_indent_code: quarterword = 0;
// @d math_surround_code=1 {space around math in text}
/// space around math in text
pub(crate) const math_surround_code: quarterword = 1;
// @d line_skip_limit_code=2 {threshold for |line_skip| instead of |baseline_skip|}
/// threshold for `line_skip` instead of `baseline_skip`
pub(crate) const line_skip_limit_code: quarterword = 2;
// @d hsize_code=3 {line width in horizontal mode}
/// line width in horizontal mode
pub(crate) const hsize_code: quarterword = 3;
// @d vsize_code=4 {page height in vertical mode}
/// page height in vertical mode
pub(crate) const vsize_code: quarterword = 4;
// @d max_depth_code=5 {maximum depth of boxes on main pages}
/// maximum depth of boxes on main pages
pub(crate) const max_depth_code: quarterword = 5;
// @d split_max_depth_code=6 {maximum depth of boxes on split pages}
/// maximum depth of boxes on split pages
pub(crate) const split_max_depth_code: quarterword = 6;
// @d box_max_depth_code=7 {maximum depth of explicit vboxes}
/// maximum depth of explicit vboxes
pub(crate) const box_max_depth_code: quarterword = 7;
// @d hfuzz_code=8 {tolerance for overfull hbox messages}
/// tolerance for overfull hbox messages
pub(crate) const hfuzz_code: quarterword = 8;
// @d vfuzz_code=9 {tolerance for overfull vbox messages}
/// tolerance for overfull vbox messages
pub(crate) const vfuzz_code: quarterword = 9;
// @d delimiter_shortfall_code=10 {maximum amount uncovered by variable delimiters}
/// maximum amount uncovered by variable delimiters
pub(crate) const delimiter_shortfall_code: quarterword = 10;
// @d null_delimiter_space_code=11 {blank space in null delimiters}
/// blank space in null delimiters
pub(crate) const null_delimiter_space_code: quarterword = 11;
// @d script_space_code=12 {extra space after subscript or superscript}
/// extra space after subscript or superscript
pub(crate) const script_space_code: quarterword = 12;
// @d pre_display_size_code=13 {length of text preceding a display}
/// length of text preceding a display
pub(crate) const pre_display_size_code: quarterword = 13;
// @d display_width_code=14 {length of line for displayed equation}
/// length of line for displayed equation
pub(crate) const display_width_code: quarterword = 14;
// @d display_indent_code=15 {indentation of line for displayed equation}
/// indentation of line for displayed equation
pub(crate) const display_indent_code: quarterword = 15;
// @d overfull_rule_code=16 {width of rule that identifies overfull hboxes}
/// width of rule that identifies overfull hboxes
pub(crate) const overfull_rule_code: quarterword = 16;
// @d hang_indent_code=17 {amount of hanging indentation}
/// amount of hanging indentation
pub(crate) const hang_indent_code: quarterword = 17;
// @d h_offset_code=18 {amount of horizontal offset when shipping pages out}
/// amount of horizontal offset when shipping pages out
pub(crate) const h_offset_code: quarterword = 18;
// @d v_offset_code=19 {amount of vertical offset when shipping pages out}
/// amount of vertical offset when shipping pages out
pub(crate) const v_offset_code: quarterword = 19;
// @d emergency_stretch_code=20 {reduces badnesses on final pass of line-breaking}
/// reduces badnesses on final pass of line-breaking
pub(crate) const emergency_stretch_code: quarterword = 20;
// @d dimen_pars=21 {total number of dimension parameters}
/// total number of dimension parameters
pub(crate) type dimen_pars_TYPENUM = U21;
pub(crate) const dimen_pars: halfword = dimen_pars_TYPENUM::U16;
// @d scaled_base=dimen_base+dimen_pars
//   {table of 256 user-defined \.{\\dimen} registers}
/// table of 256 user-defined `\dimen` registers
pub(crate) type scaled_base_TYPENUM = typenum::op!(dimen_base_TYPENUM + dimen_pars_TYPENUM);
pub(crate) const scaled_base: word = scaled_base_TYPENUM::U32;
// @d eqtb_size=scaled_base+255 {largest subscript of |eqtb|}
/// largest subscript of `eqtb`
pub(crate) type eqtb_size_TYPENUM = typenum::op!(scaled_base_TYPENUM + U255);
pub(crate) const eqtb_size: word = eqtb_size_TYPENUM::U32;
// @#
// @d dimen(#)==eqtb[scaled_base+#].sc
macro_rules! dimen {
    ($globals:expr, $val:expr) => {
        $globals.eqtb[crate::section_0247::scaled_base as crate::section_0115::pointer
            + $val as crate::section_0115::pointer][crate::section_0101::MEMORY_WORD_SC]
    };
}
// @d dimen_par(#)==eqtb[dimen_base+#].sc {a scaled quantity}
/// a scaled quantity
macro_rules! dimen_par {
    ($globals:expr, $val:expr) => {
        $globals.eqtb[crate::section_0236::dimen_base as crate::section_0115::pointer
            + $val as crate::section_0115::pointer][crate::section_0101::MEMORY_WORD_SC]
    };
}
// @d par_indent==dimen_par(par_indent_code)
// @d math_surround==dimen_par(math_surround_code)
// @d line_skip_limit==dimen_par(line_skip_limit_code)
// @d hsize==dimen_par(hsize_code)
// @d vsize==dimen_par(vsize_code)
// @d max_depth==dimen_par(max_depth_code)
// @d split_max_depth==dimen_par(split_max_depth_code)
// @d box_max_depth==dimen_par(box_max_depth_code)
macro_rules! box_max_depth {
    ($globals:expr) => {
        dimen_par!($globals, crate::section_0247::box_max_depth_code)
    };
}
// @d hfuzz==dimen_par(hfuzz_code)
// @d vfuzz==dimen_par(vfuzz_code)
// @d delimiter_shortfall==dimen_par(delimiter_shortfall_code)
// @d null_delimiter_space==dimen_par(null_delimiter_space_code)
// @d script_space==dimen_par(script_space_code)
// @d pre_display_size==dimen_par(pre_display_size_code)
// @d display_width==dimen_par(display_width_code)
// @d display_indent==dimen_par(display_indent_code)
// @d overfull_rule==dimen_par(overfull_rule_code)
// @d hang_indent==dimen_par(hang_indent_code)
macro_rules! hang_indent {
    ($globals:expr) => {
        dimen_par!($globals, crate::section_0247::hang_indent_code)
    };
}
// @d h_offset==dimen_par(h_offset_code)
// @d v_offset==dimen_par(v_offset_code)
// @d emergency_stretch==dimen_par(emergency_stretch_code)
//
// @p procedure print_length_param(@!n:integer);
pub(crate) fn print_length_param(globals: &mut TeXGlobals, n: integer) {
    // begin case n of
    if false {
        unreachable!();
    }
    // par_indent_code:print_esc("parindent");
    else if n == par_indent_code as integer {
        print_esc(globals, strpool_str!("parindent"));
    }
    // math_surround_code:print_esc("mathsurround");
    else if n == math_surround_code as integer {
        print_esc(globals, strpool_str!("mathsurround"));
    }
    // line_skip_limit_code:print_esc("lineskiplimit");
    else if n == line_skip_limit_code as integer {
        print_esc(globals, strpool_str!("lineskiplimit"));
    }
    // hsize_code:print_esc("hsize");
    else if n == hsize_code as integer {
        print_esc(globals, strpool_str!("hsize"));
    }
    // vsize_code:print_esc("vsize");
    else if n == vsize_code as integer {
        print_esc(globals, strpool_str!("vsize"));
    }
    // max_depth_code:print_esc("maxdepth");
    else if n == max_depth_code as integer {
        print_esc(globals, strpool_str!("maxdepth"));
    }
    // split_max_depth_code:print_esc("splitmaxdepth");
    else if n == split_max_depth_code as integer {
        print_esc(globals, strpool_str!("splitmaxdepth"));
    }
    // box_max_depth_code:print_esc("boxmaxdepth");
    else if n == box_max_depth_code as integer {
        print_esc(globals, strpool_str!("boxmaxdepth"));
    }
    // hfuzz_code:print_esc("hfuzz");
    else if n == hfuzz_code as integer {
        print_esc(globals, strpool_str!("hfuzz"));
    }
    // vfuzz_code:print_esc("vfuzz");
    else if n == vfuzz_code as integer {
        print_esc(globals, strpool_str!("vfuzz"));
    }
    // delimiter_shortfall_code:print_esc("delimitershortfall");
    else if n == delimiter_shortfall_code as integer {
        print_esc(globals, strpool_str!("delimitershortfall"));
    }
    // null_delimiter_space_code:print_esc("nulldelimiterspace");
    else if n == null_delimiter_space_code as integer {
        print_esc(globals, strpool_str!("nulldelimiterspace"));
    }
    // script_space_code:print_esc("scriptspace");
    else if n == script_space_code as integer {
        print_esc(globals, strpool_str!("scriptspace"));
    }
    // pre_display_size_code:print_esc("predisplaysize");
    else if n == pre_display_size_code as integer {
        print_esc(globals, strpool_str!("predisplaysize"));
    }
    // display_width_code:print_esc("displaywidth");
    else if n == display_width_code as integer {
        print_esc(globals, strpool_str!("displaywidth"));
    }
    // display_indent_code:print_esc("displayindent");
    else if n == display_indent_code as integer {
        print_esc(globals, strpool_str!("displayindent"));
    }
    // overfull_rule_code:print_esc("overfullrule");
    else if n == overfull_rule_code as integer {
        print_esc(globals, strpool_str!("overfullrule"));
    }
    // hang_indent_code:print_esc("hangindent");
    else if n == hang_indent_code as integer {
        print_esc(globals, strpool_str!("hangindent"));
    }
    // h_offset_code:print_esc("hoffset");
    else if n == h_offset_code as integer {
        print_esc(globals, strpool_str!("hoffset"));
    }
    // v_offset_code:print_esc("voffset");
    else if n == v_offset_code as integer {
        print_esc(globals, strpool_str!("voffset"));
    }
    // emergency_stretch_code:print_esc("emergencystretch");
    else if n == emergency_stretch_code as integer {
        print_esc(globals, strpool_str!("emergencystretch"));
    }
    // othercases print("[unknown dimen parameter!]")
    else {
        trace_error_expr!("n = {}", n);
        print(globals, strpool_str!("[unknown dimen parameter!]").get() as _);
    }
    // endcases;
    // end;
}

use crate::pascal::word;
use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0059::print;
use crate::section_0063::print_esc;
use crate::section_0113::halfword;
use crate::section_0113::quarterword;
use crate::section_0236::dimen_base_TYPENUM;
use typenum::Unsigned;
use typenum::{U21, U255};
