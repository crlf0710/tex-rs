//! @ By sorting the list of available spaces in the variable-size portion of
//! |mem|, we are usually able to get by without having to dump very much
//! of the dynamic memory.
//!
//! We recompute |var_used| and |dyn_used|, so that \.{INITEX} dumps valid
//! information even when it has not been gathering statistics.
//
// @<Dump the dynamic memory@>=
macro_rules! Dump_the_dynamic_memory {
    ($globals:expr) => {{
        /// all-purpose pointers
        let (mut p, mut q): (pointer, pointer);
        /// something to dump
        let mut x: integer;
        // sort_avail; var_used:=0;
        sort_avail($globals)?;
        $globals.var_used = 0;
        // dump_int(lo_mem_max); dump_int(rover);
        dump_int!($globals, $globals.lo_mem_max as _);
        dump_int!($globals, $globals.rover as _);
        // p:=mem_bot; q:=rover; x:=0;
        p = mem_bot;
        q = $globals.rover;
        x = 0;
        // repeat for k:=p to q+1 do dump_wd(mem[k]);
        loop {
            for k in p..=q+1 {
                dump_wd!($globals, $globals.mem[k]);
            }
            // x:=x+q+2-p; var_used:=var_used+q-p;
            x += q as integer + 2 - p as integer;
            $globals.var_used += q as integer - p as integer;
            // p:=q+node_size(q); q:=rlink(q);
            p = q + node_size!($globals, q);
            q = rlink!($globals, q);
            // until q=rover;
            if q == $globals.rover {
                break;
            }
        }
        // var_used:=var_used+lo_mem_max-p; dyn_used:=mem_end+1-hi_mem_min;@/
        $globals.var_used += $globals.lo_mem_max as integer - p as integer;
        $globals.dyn_used = $globals.mem_end as integer + 1 - $globals.hi_mem_min as integer;
        // for k:=p to lo_mem_max do dump_wd(mem[k]);
        for k in p ..= $globals.lo_mem_max {
            dump_wd!($globals, $globals.mem[k]);
        }
        // x:=x+lo_mem_max+1-p;
        x += $globals.lo_mem_max as integer + 1 - p as integer;
        // dump_int(hi_mem_min); dump_int(avail);
        dump_int!($globals, $globals.hi_mem_min as _);
        dump_int!($globals, $globals.avail as _);
        // for k:=hi_mem_min to mem_end do dump_wd(mem[k]);
        for k in $globals.hi_mem_min..=$globals.mem_end {
            dump_wd!($globals, $globals.mem[k]);
        }
        // x:=x+mem_end+1-hi_mem_min;
        x += $globals.mem_end as integer + 1 - $globals.hi_mem_min as integer;
        // p:=avail;
        p = $globals.avail;
        // while p<>null do
        while p != null {
            // begin decr(dyn_used); p:=link(p);
            decr!($globals.dyn_used);
            p = link!($globals, p);
            // end;
        }
        // dump_int(var_used); dump_int(dyn_used);
        dump_int!($globals, $globals.var_used);
        dump_int!($globals, $globals.dyn_used);
        // print_ln; print_int(x);
        print_ln(make_globals_io_string_log_view!($globals));
        print_int($globals, x);
        // print(" memory locations dumped; current usage is ");
        print($globals, strpool_str!(" memory locations dumped; current usage is ").get() as _);
        // print_int(var_used); print_char("&"); print_int(dyn_used)
        print_int($globals, $globals.var_used);
        print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'&'));
        print_int($globals, $globals.dyn_used);
        use crate::pascal::integer;
        use crate::section_0004::TeXGlobalsIoStringLogView;
        use crate::section_0012::mem_bot;
        use crate::section_0057::print_ln;
        use crate::section_0058::print_char;
        use crate::section_0059::print;
        use crate::section_0065::print_int;
        use crate::section_0115::pointer;
        use crate::section_0115::null;
        use crate::section_0131::sort_avail;
    }}
}