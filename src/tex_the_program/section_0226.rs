//! @ The symbolic names for glue parameters are put into \TeX's hash table
//! by using the routine called |primitive|, defined below. Let us enter them
//! now, so that we don't have to list all those parameter names anywhere else.
//
// @<Put each of \TeX's primitives into the hash table@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_0226($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("lineskip",assign_glue,glue_base+line_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("lineskip"),
        assign_glue,
        (glue_base + line_skip_code as word) as _,
    );
    // @!@:line_skip_}{\.{\\lineskip} primitive@>
    // primitive("baselineskip",assign_glue,glue_base+baseline_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("baselineskip"),
        assign_glue,
        (glue_base + baseline_skip_code as word) as _,
    );
    // @!@:baseline_skip_}{\.{\\baselineskip} primitive@>
    // primitive("parskip",assign_glue,glue_base+par_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("parskip"),
        assign_glue,
        (glue_base + par_skip_code as word) as _,
    );
    // @!@:par_skip_}{\.{\\parskip} primitive@>
    // primitive("abovedisplayskip",assign_glue,glue_base+above_display_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("abovedisplayskip"),
        assign_glue,
        (glue_base + above_display_skip_code as word) as _,
    );
    // @!@:above_display_skip_}{\.{\\abovedisplayskip} primitive@>
    // primitive("belowdisplayskip",assign_glue,glue_base+below_display_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("belowdisplayskip"),
        assign_glue,
        (glue_base + below_display_skip_code as word) as _,
    );
    // @!@:below_display_skip_}{\.{\\belowdisplayskip} primitive@>
    // primitive("abovedisplayshortskip",
    //   assign_glue,glue_base+above_display_short_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("abovedisplayshortskip"),
        assign_glue,
        (glue_base + above_display_short_skip_code as word) as _,
    );
    // @!@:above_display_short_skip_}{\.{\\abovedisplayshortskip} primitive@>
    // primitive("belowdisplayshortskip",
    //   assign_glue,glue_base+below_display_short_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("belowdisplayshortskip"),
        assign_glue,
        (glue_base + below_display_short_skip_code as word) as _,
    );
    // @!@:below_display_short_skip_}{\.{\\belowdisplayshortskip} primitive@>
    // primitive("leftskip",assign_glue,glue_base+left_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("leftskip"),
        assign_glue,
        (glue_base + left_skip_code as word) as _,
    );
    // @!@:left_skip_}{\.{\\leftskip} primitive@>
    // primitive("rightskip",assign_glue,glue_base+right_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("rightskip"),
        assign_glue,
        (glue_base + right_skip_code as word) as _,
    );
    // @!@:right_skip_}{\.{\\rightskip} primitive@>
    // primitive("topskip",assign_glue,glue_base+top_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("topskip"),
        assign_glue,
        (glue_base + top_skip_code as word) as _,
    );
    // @!@:top_skip_}{\.{\\topskip} primitive@>
    // primitive("splittopskip",assign_glue,glue_base+split_top_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("splittopskip"),
        assign_glue,
        (glue_base + split_top_skip_code as word) as _,
    );
    // @!@:split_top_skip_}{\.{\\splittopskip} primitive@>
    // primitive("tabskip",assign_glue,glue_base+tab_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("tabskip"),
        assign_glue,
        (glue_base + tab_skip_code as word) as _,
    );
    // @!@:tab_skip_}{\.{\\tabskip} primitive@>
    // primitive("spaceskip",assign_glue,glue_base+space_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("spaceskip"),
        assign_glue,
        (glue_base + space_skip_code as word) as _,
    );
    // @!@:space_skip_}{\.{\\spaceskip} primitive@>
    // primitive("xspaceskip",assign_glue,glue_base+xspace_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("xspaceskip"),
        assign_glue,
        (glue_base + xspace_skip_code as word) as _,
    );
    // @!@:xspace_skip_}{\.{\\xspaceskip} primitive@>
    // primitive("parfillskip",assign_glue,glue_base+par_fill_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("parfillskip"),
        assign_glue,
        (glue_base + par_fill_skip_code as word) as _,
    );
    // @!@:par_fill_skip_}{\.{\\parfillskip} primitive@>
    // primitive("thinmuskip",assign_mu_glue,glue_base+thin_mu_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("thinmuskip"),
        assign_mu_glue,
        (glue_base + thin_mu_skip_code as word) as _,
    );
    // @!@:thin_mu_skip_}{\.{\\thinmuskip} primitive@>
    // primitive("medmuskip",assign_mu_glue,glue_base+med_mu_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("medmuskip"),
        assign_mu_glue,
        (glue_base + med_mu_skip_code as word) as _,
    );
    // @!@:med_mu_skip_}{\.{\\medmuskip} primitive@>
    // primitive("thickmuskip",assign_mu_glue,glue_base+thick_mu_skip_code);@/
    primitive(
        globals,
        crate::strpool_str!("thickmuskip"),
        assign_mu_glue,
        (glue_base + thick_mu_skip_code as word) as _,
    );
    // @!@:thick_mu_skip_}{\.{\\thickmuskip} primitive@>
}}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0209::assign_glue;
use crate::section_0209::assign_mu_glue;
use crate::section_0222::glue_base;
use crate::section_0224::*;
use crate::section_0264::primitive;
