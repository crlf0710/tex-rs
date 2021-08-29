//! ` `

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0248(globals: &mut TeXGlobals) {
    // primitive("parindent",assign_dimen,dimen_base+par_indent_code);@/
    primitive(globals, strpool_str!("parindent"), assign_dimen, (dimen_base + par_indent_code as word) as _);
    // @!@:par_indent_}{\.{\\parindent} primitive@>
    // primitive("mathsurround",assign_dimen,dimen_base+math_surround_code);@/
    primitive(globals, strpool_str!("mathsurround"), assign_dimen, (dimen_base + math_surround_code as word) as _);
    // @!@:math_surround_}{\.{\\mathsurround} primitive@>
    // primitive("lineskiplimit",assign_dimen,dimen_base+line_skip_limit_code);@/
    primitive(globals, strpool_str!("lineskiplimit"), assign_dimen, (dimen_base + line_skip_limit_code as word) as _);
    // @!@:line_skip_limit_}{\.{\\lineskiplimit} primitive@>
    // primitive("hsize",assign_dimen,dimen_base+hsize_code);@/
    primitive(globals, strpool_str!("hsize"), assign_dimen, (dimen_base + hsize_code as word) as _);
    // @!@:hsize_}{\.{\\hsize} primitive@>
    // primitive("vsize",assign_dimen,dimen_base+vsize_code);@/
    primitive(globals, strpool_str!("vsize"), assign_dimen, (dimen_base + vsize_code as word) as _);
    // @!@:vsize_}{\.{\\vsize} primitive@>
    // primitive("maxdepth",assign_dimen,dimen_base+max_depth_code);@/
    primitive(globals, strpool_str!("maxdepth"), assign_dimen, (dimen_base + max_depth_code as word) as _);
    // @!@:max_depth_}{\.{\\maxdepth} primitive@>
    // primitive("splitmaxdepth",assign_dimen,dimen_base+split_max_depth_code);@/
    primitive(globals, strpool_str!("splitmaxdepth"), assign_dimen, (dimen_base + split_max_depth_code as word) as _);
    // @!@:split_max_depth_}{\.{\\splitmaxdepth} primitive@>
    // primitive("boxmaxdepth",assign_dimen,dimen_base+box_max_depth_code);@/
    primitive(globals, strpool_str!("boxmaxdepth"), assign_dimen, (dimen_base + box_max_depth_code as word) as _);
    // @!@:box_max_depth_}{\.{\\boxmaxdepth} primitive@>
    // primitive("hfuzz",assign_dimen,dimen_base+hfuzz_code);@/
    primitive(globals, strpool_str!("hfuzz"), assign_dimen, (dimen_base + hfuzz_code as word) as _);
    // @!@:hfuzz_}{\.{\\hfuzz} primitive@>
    // primitive("vfuzz",assign_dimen,dimen_base+vfuzz_code);@/
    primitive(globals, strpool_str!("vfuzz"), assign_dimen, (dimen_base + vfuzz_code as word) as _);
    // @!@:vfuzz_}{\.{\\vfuzz} primitive@>
    // primitive("delimitershortfall",
    //   assign_dimen,dimen_base+delimiter_shortfall_code);@/
    primitive(globals, strpool_str!("delimitershortfall"), assign_dimen, (dimen_base + delimiter_shortfall_code as word) as _);
    // @!@:delimiter_shortfall_}{\.{\\delimitershortfall} primitive@>
    // primitive("nulldelimiterspace",
    //   assign_dimen,dimen_base+null_delimiter_space_code);@/
    primitive(globals, strpool_str!("nulldelimiterspace"), assign_dimen, (dimen_base + null_delimiter_space_code as word) as _);
    // @!@:null_delimiter_space_}{\.{\\nulldelimiterspace} primitive@>
    // primitive("scriptspace",assign_dimen,dimen_base+script_space_code);@/
    primitive(globals, strpool_str!("scriptspace"), assign_dimen, (dimen_base + script_space_code as word) as _);
    // @!@:script_space_}{\.{\\scriptspace} primitive@>
    // primitive("predisplaysize",assign_dimen,dimen_base+pre_display_size_code);@/
    primitive(globals, strpool_str!("predisplaysize"), assign_dimen, (dimen_base + pre_display_size_code as word) as _);
    // @!@:pre_display_size_}{\.{\\predisplaysize} primitive@>
    // primitive("displaywidth",assign_dimen,dimen_base+display_width_code);@/
    primitive(globals, strpool_str!("displaywidth"), assign_dimen, (dimen_base + display_width_code as word) as _);
    // @!@:display_width_}{\.{\\displaywidth} primitive@>
    // primitive("displayindent",assign_dimen,dimen_base+display_indent_code);@/
    primitive(globals, strpool_str!("displayindent"), assign_dimen, (dimen_base + display_indent_code as word) as _);
    // @!@:display_indent_}{\.{\\displayindent} primitive@>
    // primitive("overfullrule",assign_dimen,dimen_base+overfull_rule_code);@/
    primitive(globals, strpool_str!("overfullrule"), assign_dimen, (dimen_base + overfull_rule_code as word) as _);
    // @!@:overfull_rule_}{\.{\\overfullrule} primitive@>
    // primitive("hangindent",assign_dimen,dimen_base+hang_indent_code);@/
    primitive(globals, strpool_str!("hangindent"), assign_dimen, (dimen_base + hang_indent_code as word) as _);
    // @!@:hang_indent_}{\.{\\hangindent} primitive@>
    // primitive("hoffset",assign_dimen,dimen_base+h_offset_code);@/
    primitive(globals, strpool_str!("hoffset"), assign_dimen, (dimen_base + h_offset_code as word) as _);
    // @!@:h_offset_}{\.{\\hoffset} primitive@>
    // primitive("voffset",assign_dimen,dimen_base+v_offset_code);@/
    primitive(globals, strpool_str!("voffset"), assign_dimen, (dimen_base + v_offset_code as word) as _);
    // @!@:v_offset_}{\.{\\voffset} primitive@>
    // primitive("emergencystretch",assign_dimen,dimen_base+emergency_stretch_code);@/
    primitive(globals, strpool_str!("emergencystretch"), assign_dimen, (dimen_base + emergency_stretch_code as word) as _);
    // @!@:emergency_stretch_}{\.{\\emergencystretch} primitive@>
}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0209::assign_dimen;
use crate::section_0236::dimen_base;
use crate::section_0247::*;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
