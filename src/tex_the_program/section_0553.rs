//! ` `

// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_0553($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("nullfont",set_font,null_font);
    primitive(
        globals,
        crate::strpool_str!("nullfont"),
        set_font,
        null_font.get() as _,
    );
    // @!@:null_font_}{\.{\\nullfont} primitive@>
    // text(frozen_null_font):="nullfont"; eqtb[frozen_null_font]:=eqtb[cur_val];
    text!(globals, frozen_null_font as pointer) = crate::strpool_str!("nullfont").get() as _;
    globals.eqtb[frozen_null_font as pointer] = globals.eqtb[globals.cur_val as pointer];
}}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0209::set_font;
use crate::section_0222::frozen_null_font;
use crate::section_0232::null_font;
use crate::section_0256::text;
use crate::section_0264::primitive;
