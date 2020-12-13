//! @ Similarly, |delete_glue_ref| is called when a pointer to a glue
//! specification is being withdrawn.
//! @^reference counts@>

// @d fast_delete_glue_ref(#)==@t@>@;@/
macro_rules! fast_delete_glue_ref {
    ($globals:expr, $ptr:expr) => {{
        // begin if glue_ref_count(#)=null then free_node(#,glue_spec_size)
        if glue_ref_count!($globals, $ptr) == null {
            free_node($globals, $ptr, glue_spec_size as _);
        }
        // else decr(glue_ref_count(#));
        else {
            decr!(glue_ref_count!($globals, $ptr))
        }
        // end
        use crate::section_0115::null;
        use crate::section_0130::free_node;
        use crate::section_0150::glue_spec_size;
    }}
}

// @p procedure delete_glue_ref(@!p:pointer); {|p| points to a glue specification}
/// `p` points to a glue specification
pub(crate) fn delete_glue_ref(globals: &mut TeXGlobals, p: pointer) {
    // fast_delete_glue_ref(p);
    fast_delete_glue_ref!(globals, p);
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
