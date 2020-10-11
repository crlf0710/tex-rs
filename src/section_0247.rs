//! @ The final region of |eqtb| contains the dimension parameters defined
//! here, and the 256 \.{\\dimen} registers.
//
// @d par_indent_code=0 {indentation of paragraphs}
// @d math_surround_code=1 {space around math in text}
// @d line_skip_limit_code=2 {threshold for |line_skip| instead of |baseline_skip|}
// @d hsize_code=3 {line width in horizontal mode}
// @d vsize_code=4 {page height in vertical mode}
// @d max_depth_code=5 {maximum depth of boxes on main pages}
// @d split_max_depth_code=6 {maximum depth of boxes on split pages}
// @d box_max_depth_code=7 {maximum depth of explicit vboxes}
// @d hfuzz_code=8 {tolerance for overfull hbox messages}
// @d vfuzz_code=9 {tolerance for overfull vbox messages}
// @d delimiter_shortfall_code=10 {maximum amount uncovered by variable delimiters}
// @d null_delimiter_space_code=11 {blank space in null delimiters}
// @d script_space_code=12 {extra space after subscript or superscript}
// @d pre_display_size_code=13 {length of text preceding a display}
// @d display_width_code=14 {length of line for displayed equation}
// @d display_indent_code=15 {indentation of line for displayed equation}
// @d overfull_rule_code=16 {width of rule that identifies overfull hboxes}
// @d hang_indent_code=17 {amount of hanging indentation}
// @d h_offset_code=18 {amount of horizontal offset when shipping pages out}
// @d v_offset_code=19 {amount of vertical offset when shipping pages out}
// @d emergency_stretch_code=20 {reduces badnesses on final pass of line-breaking}
// @d dimen_pars=21 {total number of dimension parameters}
/// total number of dimension parameters
pub(crate) const dimen_pars: halfword = 21;
// @d scaled_base=dimen_base+dimen_pars
//   {table of 256 user-defined \.{\\dimen} registers}
/// table of 256 user-defined `\dimen` registers
pub(crate) const scaled_base: word = dimen_base + dimen_pars as word;
// @d eqtb_size=scaled_base+255 {largest subscript of |eqtb|}
/// largest subscript of `eqtb`
pub(crate) const eqtb_size: word = scaled_base + 255;
// @#
// @d dimen(#)==eqtb[scaled_base+#].sc
// @d dimen_par(#)==eqtb[dimen_base+#].sc {a scaled quantity}
// @d par_indent==dimen_par(par_indent_code)
// @d math_surround==dimen_par(math_surround_code)
// @d line_skip_limit==dimen_par(line_skip_limit_code)
// @d hsize==dimen_par(hsize_code)
// @d vsize==dimen_par(vsize_code)
// @d max_depth==dimen_par(max_depth_code)
// @d split_max_depth==dimen_par(split_max_depth_code)
// @d box_max_depth==dimen_par(box_max_depth_code)
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
// @d h_offset==dimen_par(h_offset_code)
// @d v_offset==dimen_par(v_offset_code)
// @d emergency_stretch==dimen_par(emergency_stretch_code)
//
// @p procedure print_length_param(@!n:integer);
// begin case n of
// par_indent_code:print_esc("parindent");
// math_surround_code:print_esc("mathsurround");
// line_skip_limit_code:print_esc("lineskiplimit");
// hsize_code:print_esc("hsize");
// vsize_code:print_esc("vsize");
// max_depth_code:print_esc("maxdepth");
// split_max_depth_code:print_esc("splitmaxdepth");
// box_max_depth_code:print_esc("boxmaxdepth");
// hfuzz_code:print_esc("hfuzz");
// vfuzz_code:print_esc("vfuzz");
// delimiter_shortfall_code:print_esc("delimitershortfall");
// null_delimiter_space_code:print_esc("nulldelimiterspace");
// script_space_code:print_esc("scriptspace");
// pre_display_size_code:print_esc("predisplaysize");
// display_width_code:print_esc("displaywidth");
// display_indent_code:print_esc("displayindent");
// overfull_rule_code:print_esc("overfullrule");
// hang_indent_code:print_esc("hangindent");
// h_offset_code:print_esc("hoffset");
// v_offset_code:print_esc("voffset");
// emergency_stretch_code:print_esc("emergencystretch");
// othercases print("[unknown dimen parameter!]")
// endcases;
// end;

use crate::pascal::word;
use crate::section_0113::halfword;
use crate::section_0236::dimen_base;
