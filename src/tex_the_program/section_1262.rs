//! ` `
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1262(globals: &mut TeXGlobals) {
    // primitive("batchmode",set_interaction,batch_mode);
    primitive(
        globals,
        crate::strpool_str!("batchmode"),
        set_interaction,
        batch_mode as _,
    );
    // @!@:batch_mode_}{\.{\\batchmode} primitive@>
    // primitive("nonstopmode",set_interaction,nonstop_mode);
    primitive(
        globals,
        crate::strpool_str!("nonstopmode"),
        set_interaction,
        nonstop_mode as _,
    );
    // @!@:nonstop_mode_}{\.{\\nonstopmode} primitive@>
    // primitive("scrollmode",set_interaction,scroll_mode);
    primitive(
        globals,
        crate::strpool_str!("scrollmode"),
        set_interaction,
        scroll_mode as _,
    );
    // @!@:scroll_mode_}{\.{\\scrollmode} primitive@>
    // primitive("errorstopmode",set_interaction,error_stop_mode);
    primitive(
        globals,
        crate::strpool_str!("errorstopmode"),
        set_interaction,
        error_stop_mode as _,
    );
    // @!@:error_stop_mode_}{\.{\\errorstopmode} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0073::batch_mode;
use crate::section_0073::error_stop_mode;
use crate::section_0073::nonstop_mode;
use crate::section_0073::scroll_mode;
use crate::section_0209::set_interaction;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
