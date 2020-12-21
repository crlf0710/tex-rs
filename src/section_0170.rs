//! ` `

// @<Check flags...@>=
#[allow(unused_macros)]
macro_rules! Check_flags_of_unavailable_nodes {
    ($globals:expr) => {{
        /// current locations of interest in `mem`
        let mut p: pointer;
        // p:=mem_min;
        p = mem_min as pointer;
        // while p<=lo_mem_max do {node |p| should not be empty}
        /// node `p` should not be empty
        while p <= $globals.lo_mem_max {
            // begin if is_empty(p) then
            if is_empty!($globals, p) {
                // begin print_nl("Bad flag at "); print_int(p);
                print_nl($globals, strpool_str!("Bad flag at "));
                print_int($globals, p as _);
                // @.Bad flag...@>
                // end;
            }
            // while (p<=lo_mem_max) and not free[p] do incr(p);
            while p <= $globals.lo_mem_max && !$globals.free[p] {
                incr!(p);
            }
            // while (p<=lo_mem_max) and free[p] do incr(p);
            while p <= $globals.lo_mem_max && $globals.free[p] {
                incr!(p);
            }
            // end
        }
        use crate::section_0062::print_nl;
        use crate::section_0065::print_int;
    }}
}
