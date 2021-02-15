//! @ The lower part of |mem| grows by 1000 words at a time, unless
//! we are very close to going under. When it grows, we simply link
//! a new node into the available-space list. This method of controlled
//! growth helps to keep the |mem| usage consecutive when \TeX\ is
//! implemented on ``virtual memory'' systems.
//! @^virtual memory@>
//
// @<Grow more variable-size memory and |goto restart|@>=
macro_rules! Grow_more_variable_size_memory_and_goto_restart {
    ($globals:expr, $p:expr, $q:expr, $lbl_restart:lifetime) => {{
        /// temporary register
        let mut t: integer;
        // begin if hi_mem_min-lo_mem_max>=1998 then t:=lo_mem_max+1000
        if $globals.hi_mem_min - $globals.lo_mem_max >= 1998 {
            t = ($globals.lo_mem_max + 1000) as integer;
        }
        // else t:=lo_mem_max+1+(hi_mem_min-lo_mem_max) div 2;
        // {|lo_mem_max+2<=t<hi_mem_min|}
        else {
            t = ($globals.lo_mem_max + 1 + ($globals.hi_mem_min - $globals.lo_mem_max) / 2)
                as integer;
            /// `lo_mem_max + 2 <= t < hi_mem_min`
            const _: () = ();
        }
        // p:=llink(rover); q:=lo_mem_max; rlink(p):=q; llink(rover):=q;@/
        $p = llink!($globals, $globals.rover);
        $q = $globals.lo_mem_max;
        rlink!($globals, $p) = $q;
        llink!($globals, $globals.rover) = $q;
        // if t>mem_bot+max_halfword then t:=mem_bot+max_halfword;
        if t > (mem_bot + max_halfword) as integer {
            t = (mem_bot + max_halfword) as integer;
        }
        // rlink(q):=rover; llink(q):=p; link(q):=empty_flag; node_size(q):=t-lo_mem_max;@/
        rlink!($globals, $q) = $globals.rover;
        llink!($globals, $q) = $p;
        link!($globals, $q) = empty_flag;
        node_size!($globals, $q) = t as halfword - $globals.lo_mem_max;
        // lo_mem_max:=t; link(lo_mem_max):=null; info(lo_mem_max):=null;
        $globals.lo_mem_max = t as _;
        link!($globals, $globals.lo_mem_max) = null;
        info_inner!($globals, $globals.lo_mem_max) = null;
        // rover:=q; goto restart;
        $globals.rover = $q;
        goto_backward_label!($lbl_restart);
        // end
        use crate::section_0113::halfword;
        use crate::section_0124::empty_flag;
    }};
}
