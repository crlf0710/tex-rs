//! @ Conversely, a one-word node is recycled by calling |free_avail|.
//! This routine is part of \TeX's ``inner loop,'' so we want it to be fast.
//! @^inner loop@>
//
// @d free_avail(#)== {single-word node liberation}
/// single-word node liberation
pub(crate) macro free_avail($globals:expr, $ptr:expr) {{
    // begin link(#):=avail; avail:=#;
    link!($globals, $ptr) = $globals.avail;
    $globals.avail = $ptr;
    // @!stat decr(dyn_used);@+tats@/
    crate::region_stat! {
        decr!($globals.dyn_used);
    }
    // end
    use crate::section_0118::link;
}}
