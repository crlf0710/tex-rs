//! @ The hash table is initialized with `\.{\\count}', `\.{\\dimen}', `\.{\\skip}',
//! and `\.{\\muskip}' all having |register| as their command code; they are
//! distinguished by the |chr_code|, which is either |int_val|, |dimen_val|,
//! |glue_val|, or |mu_val|.
//
// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_0411($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("count",register,int_val);
    primitive(
        globals,
        crate::strpool_str!("count"),
        register,
        int_val as _,
    );
    // @!@:count_}{\.{\\count} primitive@>
    // primitive("dimen",register,dimen_val);
    primitive(
        globals,
        crate::strpool_str!("dimen"),
        register,
        dimen_val as _,
    );
    // @!@:dimen_}{\.{\\dimen} primitive@>
    // primitive("skip",register,glue_val);
    primitive(
        globals,
        crate::strpool_str!("skip"),
        register,
        glue_val as _,
    );
    // @!@:skip_}{\.{\\skip} primitive@>
    // primitive("muskip",register,mu_val);
    primitive(
        globals,
        crate::strpool_str!("muskip"),
        register,
        mu_val as _,
    );
    // @!@:mu_skip_}{\.{\\muskip} primitive@>
    use cur_val_level_kind::*;
}}

use crate::section_0004::TeXGlobals;
use crate::section_0209::register;
use crate::section_0264::primitive;
use crate::section_0410::cur_val_level_kind;
