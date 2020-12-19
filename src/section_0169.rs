//! ` `

// @<Check variable-size...@>=
macro_rules! Check_variable_size_avail_list {
    ($globals:expr) => {{
        /// current locations of interest in `mem`
        let (mut p, mut q) : (pointer, pointer);
        /// is something amiss?
        let mut clobbered: boolean;
        // p:=rover; q:=null; clobbered:=false;
        p = $globals.rover;
        q = null;
        clobbered = false;
        region_forward_label!(
        |'done2|
        {
        // repeat if (p>=lo_mem_max)or(p<mem_min) then clobbered:=true
        loop {
            if p >= $globals.lo_mem_max || p < mem_min as pointer {
                clobbered = true;
            }
            // else if (rlink(p)>=lo_mem_max)or(rlink(p)<mem_min) then clobbered:=true
            else if rlink!($globals, p) >= $globals.lo_mem_max ||
                rlink!($globals, p) < mem_min as pointer {
                clobbered = true;
            }
            // else if  not(is_empty(p))or(node_size(p)<2)or@|
            //  (p+node_size(p)>lo_mem_max)or@| (llink(rlink(p))<>p) then clobbered:=true;
            else if !is_empty!($globals, p) || node_size!($globals, p) < 2 ||
                p + node_size!($globals, p) > $globals.lo_mem_max ||
                llink!($globals, rlink!($globals, p)) != p {
                clobbered = true;
            }
            // if clobbered then
            if clobbered {
                // begin print_nl("Double-AVAIL list clobbered at ");
                print_nl($globals, strpool_str!("Double-AVAIL list clobbered at "));
                // print_int(q); goto done2;
                print_int($globals, q as _);
                goto_forward_label!('done2);
                // end;
            }
            // for q:=p to p+node_size(p)-1 do {mark all locations free}
            /// mark all locations free
            for q in p..= p + node_size!($globals, p) - 1 {
                // begin if free[q] then
                if $globals.free[q] {
                    // begin print_nl("Doubly free location at ");
                    print_nl($globals, strpool_str!("Doubly free location at "));
                    // @.Doubly free location...@>
                    // print_int(q); goto done2;
                    print_int($globals, q as _);
                    goto_forward_label!('done2);
                    // end;
                }
                // free[q]:=true;
                $globals.free[q] = true;
                // end;
            }
            // q:=p; p:=rlink(p);
            q = p;
            p = rlink!($globals, p);
            // until p=rover;
            if (p == $globals.rover) {
                break;
            }
        }
        }
        // done2:
        'done2 <-
        );
        use crate::section_0062::print_nl;
        use crate::section_0065::print_int;
        use crate::section_0115::null;
        use crate::section_0124::empty_flag;
    }}
}
