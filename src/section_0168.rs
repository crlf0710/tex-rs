//! ` `

// @<Check single-word...@>=
macro_rules! Check_single_word_avail_list {
    ($globals:expr) => {{
        /// current locations of interest in `mem`
        let (mut p, mut q) : (pointer, pointer);
        /// is something amiss?
        let mut clobbered: boolean;
        // p:=avail; q:=null; clobbered:=false;
        p = $globals.avail;
        q = null;
        clobbered = false;
        region_forward_label!(
        |'done1|
        {
        // while p<>null do
        while p != null {
            // begin if (p>mem_end)or(p<hi_mem_min) then clobbered:=true
            if p > $globals.mem_end || p < $globals.hi_mem_min {
                clobbered = true;
            }
            // else if free[p] then clobbered:=true;
            else if $globals.free[p] {
                clobbered = true;
            }
            // if clobbered then
            if clobbered {
                // begin print_nl("AVAIL list clobbered at ");
                print_nl($globals, strpool_str!("AVAIL list clobbered at "));
                // @.AVAIL list clobbered...@>
                // print_int(q); goto done1;
                print_int($globals, q as _);
                goto_forward_label!('done1);
                // end;
            }
            // free[p]:=true; q:=p; p:=link(q);
            $globals.free[p] = true;
            q = p;
            p = link!($globals, q);
            // end;
        }
        }
        // done1:
        'done1 <-
        );

        use crate::section_0062::print_nl;
        use crate::section_0065::print_int;
        use crate::section_0115::null;
    }}
}