//! ` `

// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_0384($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("topmark",top_bot_mark,top_mark_code);
    primitive(
        globals,
        crate::strpool_str!("topmark"),
        top_bot_mark,
        mark_code_kind::top_mark_code as _,
    );
    // @!@:top_mark_}{\.{\\topmark} primitive@>
    // primitive("firstmark",top_bot_mark,first_mark_code);
    primitive(
        globals,
        crate::strpool_str!("firstmark"),
        top_bot_mark,
        mark_code_kind::first_mark_code as _,
    );
    // @!@:first_mark_}{\.{\\firstmark} primitive@>
    // primitive("botmark",top_bot_mark,bot_mark_code);
    primitive(
        globals,
        crate::strpool_str!("botmark"),
        top_bot_mark,
        mark_code_kind::bot_mark_code as _,
    );
    // @!@:bot_mark_}{\.{\\botmark} primitive@>
    // primitive("splitfirstmark",top_bot_mark,split_first_mark_code);
    primitive(
        globals,
        crate::strpool_str!("splitfirstmark"),
        top_bot_mark,
        mark_code_kind::split_first_mark_code as _,
    );
    // @!@:split_first_mark_}{\.{\\splitfirstmark} primitive@>
    // primitive("splitbotmark",top_bot_mark,split_bot_mark_code);
    primitive(
        globals,
        crate::strpool_str!("splitbotmark"),
        top_bot_mark,
        mark_code_kind::split_bot_mark_code as _,
    );
    // @!@:split_bot_mark_}{\.{\\splitbotmark} primitive@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0210::top_bot_mark;
use crate::section_0264::primitive;
use crate::section_0382::mark_code_kind;
