//! @ Many of \TeX's primitives need no |equiv|, since they are identifiable
//! by their |eq_type| alone. These primitives are loaded into the hash table
//! as follows:

// @<Put each of \TeX's primitives into the hash table@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0265(globals: &mut TeXGlobals) {
    // primitive(" ",ex_space,0);@/
    primitive(globals, strpool_str!(" "), ex_space, 0);
    // @!@:Single-character primitives /}{\quad\.{\\\ }@>
    // primitive("/",ital_corr,0);@/
    // @!@:Single-character primitives /}{\quad\.{\\/}@>
    // primitive("accent",accent,0);@/
    // @!@:accent_}{\.{\\accent} primitive@>
    // primitive("advance",advance,0);@/
    primitive(globals, strpool_str!("advance"), advance, 0);
    // @!@:advance_}{\.{\\advance} primitive@>
    // primitive("afterassignment",after_assignment,0);@/
    // @!@:after_assignment_}{\.{\\afterassignment} primitive@>
    // primitive("aftergroup",after_group,0);@/
    primitive(globals, strpool_str!("aftergroup"), after_group, 0);
    // @!@:after_group_}{\.{\\aftergroup} primitive@>
    // primitive("begingroup",begin_group,0);@/
    primitive(globals, strpool_str!("begingroup"), begin_group, 0);
    // @!@:begin_group_}{\.{\\begingroup} primitive@>
    // primitive("char",char_num,0);@/
    // @!@:char_}{\.{\\char} primitive@>
    // primitive("csname",cs_name,0);@/
    primitive(globals, strpool_str!("csname"), cs_name, 0);
    // @!@:cs_name_}{\.{\\csname} primitive@>
    // primitive("delimiter",delim_num,0);@/
    // @!@:delimiter_}{\.{\\delimiter} primitive@>
    // primitive("divide",divide,0);@/
    primitive(globals, strpool_str!("divide"), divide, 0);
    // @!@:divide_}{\.{\\divide} primitive@>
    // primitive("endcsname",end_cs_name,0);@/
    primitive(globals, strpool_str!("endcsname"), end_cs_name, 0);
    // @!@:end_cs_name_}{\.{\\endcsname} primitive@>
    // primitive("endgroup",end_group,0);
    primitive(globals, strpool_str!("endgroup"), end_group, 0);
    // @!@:end_group_}{\.{\\endgroup} primitive@>
    // text(frozen_end_group):="endgroup"; eqtb[frozen_end_group]:=eqtb[cur_val];@/
    text!(globals, frozen_end_group as u16) = strpool_str!("endgroup").get() as _;
    globals.eqtb[frozen_end_group as u16] = globals.eqtb[globals.cur_val as u16];
    // primitive("expandafter",expand_after,0);@/
    primitive(globals, strpool_str!("expandafter"), expand_after, 0);
    // @!@:expand_after_}{\.{\\expandafter} primitive@>
    // primitive("font",def_font,0);@/
    primitive(globals, strpool_str!("font"), def_font, 0);
    // @!@:font_}{\.{\\font} primitive@>
    // primitive("fontdimen",assign_font_dimen,0);@/
    primitive(globals, strpool_str!("fontdimen"), assign_font_dimen, 0);
    // @!@:font_dimen_}{\.{\\fontdimen} primitive@>
    // primitive("halign",halign,0);@/
    // @!@:halign_}{\.{\\halign} primitive@>
    // primitive("hrule",hrule,0);@/
    // @!@:hrule_}{\.{\\hrule} primitive@>
    // primitive("ignorespaces",ignore_spaces,0);@/
    // @!@:ignore_spaces_}{\.{\\ignorespaces} primitive@>
    // primitive("insert",insert,0);@/
    // @!@:insert_}{\.{\\insert} primitive@>
    // primitive("mark",mark,0);@/
    // @!@:mark_}{\.{\\mark} primitive@>
    // primitive("mathaccent",math_accent,0);@/
    // @!@:math_accent_}{\.{\\mathaccent} primitive@>
    // primitive("mathchar",math_char_num,0);@/
    // @!@:math_char_}{\.{\\mathchar} primitive@>
    // primitive("mathchoice",math_choice,0);@/
    // @!@:math_choice_}{\.{\\mathchoice} primitive@>
    // primitive("multiply",multiply,0);@/
    primitive(globals, strpool_str!("multiply"), multiply, 0);
    // @!@:multiply_}{\.{\\multiply} primitive@>
    // primitive("noalign",no_align,0);@/
    // @!@:no_align_}{\.{\\noalign} primitive@>
    // primitive("noboundary",no_boundary,0);@/
    primitive(globals, strpool_str!("noboundary"), no_boundary, 0);
    // @!@:no_boundary_}{\.{\\noboundary} primitive@>
    // primitive("noexpand",no_expand,0);@/
    primitive(globals, strpool_str!("noexpand"), no_expand, 0);
    // @!@:no_expand_}{\.{\\noexpand} primitive@>
    // primitive("nonscript",non_script,0);@/
    // @!@:non_script_}{\.{\\nonscript} primitive@>
    // primitive("omit",omit,0);@/
    // @!@:omit_}{\.{\\omit} primitive@>
    // primitive("parshape",set_shape,0);@/
    // @!@:par_shape_}{\.{\\parshape} primitive@>
    // primitive("penalty",break_penalty,0);@/
    primitive(globals, strpool_str!("penalty"), break_penalty, 0);
    // primitive("prevgraf",set_prev_graf,0);@/
    // @!@:prev_graf_}{\.{\\prevgraf} primitive@>
    // primitive("radical",radical,0);@/
    // @!@:radical_}{\.{\\radical} primitive@>
    // @!@:penalty_}{\.{\\penalty} primitive@>
    // primitive("read",read_to_cs,0);@/
    primitive(globals, strpool_str!("read"), read_to_cs, 0);
    // @!@:read_}{\.{\\read} primitive@>
    // primitive("relax",relax,256); {cf.\ |scan_file_name|}
    primitive(globals, strpool_str!("relax"), relax, 256);
    // @!@:relax_}{\.{\\relax} primitive@>
    // text(frozen_relax):="relax"; eqtb[frozen_relax]:=eqtb[cur_val];@/
    text!(globals, frozen_relax as pointer) = strpool_str!("relax").get() as _;
    globals.eqtb[frozen_relax as pointer] = globals.eqtb[globals.cur_val as pointer];
    // primitive("setbox",set_box,0);@/
    primitive(globals, strpool_str!("setbox"), set_box, 0);
    // @!@:set_box_}{\.{\\setbox} primitive@>
    // primitive("the",the,0);@/
    primitive(globals, strpool_str!("the"), the, 0);
    // @!@:the_}{\.{\\the} primitive@>
    // primitive("toks",toks_register,0);@/
    // @!@:toks_}{\.{\\toks} primitive@>
    // primitive("vadjust",vadjust,0);@/
    primitive(globals, strpool_str!("vadjust"), vadjust, 0);
    // @!@:vadjust_}{\.{\\vadjust} primitive@>
    // primitive("valign",valign,0);@/
    // @!@:valign_}{\.{\\valign} primitive@>
    // primitive("vcenter",vcenter,0);@/
    // @!@:vcenter_}{\.{\\vcenter} primitive@>
    // primitive("vrule",vrule,0);@/
    primitive(globals, strpool_str!("vrule"), vrule, 0);
    // @!@:vrule_}{\.{\\vrule} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0207::*;
use crate::section_0208::*;
use crate::section_0209::*;
use crate::section_0210::*;
use crate::section_0222::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
