//! @ The various character code tables are changed by the |def_code| commands,
//! and the font families are declared by |def_family|.

// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1230($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("catcode",def_code,cat_code_base);
    primitive(
        globals,
        crate::strpool_str!("catcode"),
        def_code,
        cat_code_base as _,
    );
    // @!@:cat_code_}{\.{\\catcode} primitive@>
    // primitive("mathcode",def_code,math_code_base);
    primitive(
        globals,
        crate::strpool_str!("mathcode"),
        def_code,
        math_code_base as _,
    );
    // @!@:math_code_}{\.{\\mathcode} primitive@>
    // primitive("lccode",def_code,lc_code_base);
    primitive(
        globals,
        crate::strpool_str!("lccode"),
        def_code,
        lc_code_base as _,
    );
    // @!@:lc_code_}{\.{\\lccode} primitive@>
    // primitive("uccode",def_code,uc_code_base);
    primitive(
        globals,
        crate::strpool_str!("uccode"),
        def_code,
        uc_code_base as _,
    );
    // @!@:uc_code_}{\.{\\uccode} primitive@>
    // primitive("sfcode",def_code,sf_code_base);
    primitive(
        globals,
        crate::strpool_str!("sfcode"),
        def_code,
        sf_code_base as _,
    );
    // @!@:sf_code_}{\.{\\sfcode} primitive@>
    // primitive("delcode",def_code,del_code_base);
    primitive(
        globals,
        crate::strpool_str!("delcode"),
        def_code,
        del_code_base as _,
    );
    // @!@:del_code_}{\.{\\delcode} primitive@>
    // primitive("textfont",def_family,math_font_base);
    primitive(
        globals,
        crate::strpool_str!("textfont"),
        def_family,
        math_font_base as _,
    );
    // @!@:text_font_}{\.{\\textfont} primitive@>
    // primitive("scriptfont",def_family,math_font_base+script_size);
    primitive(
        globals,
        crate::strpool_str!("scriptfont"),
        def_family,
        (math_font_base + script_size as word) as _,
    );
    // @!@:script_font_}{\.{\\scriptfont} primitive@>
    // primitive("scriptscriptfont",def_family,math_font_base+script_script_size);
    primitive(
        globals,
        crate::strpool_str!("scriptscriptfont"),
        def_family,
        (math_font_base + script_script_size as word) as _,
    );
    // @!@:script_script_font_}{\.{\\scriptscriptfont} primitive@>
}}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0209::*;
use crate::section_0230::cat_code_base;
use crate::section_0230::lc_code_base;
use crate::section_0230::math_code_base;
use crate::section_0230::math_font_base;
use crate::section_0230::sf_code_base;
use crate::section_0230::uc_code_base;
use crate::section_0236::del_code_base;
use crate::section_0264::primitive;
use crate::section_0699::script_script_size;
use crate::section_0699::script_size;
