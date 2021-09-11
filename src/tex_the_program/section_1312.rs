//! ` `
// @<Undump the dynamic memory@>=
pub(crate) macro Undump_the_dynamic_memory($globals:expr, $lbl_bad_fmt:lifetime) {{
    /// all-purpose pointers
    let (mut p, mut q): (pointer, pointer);
    // undump(lo_mem_stat_max+1000)(hi_mem_stat_min-1)(lo_mem_max);
    undump!(
        $globals,
        lo_mem_stat_max + 1000,
        hi_mem_stat_min - 1,
        $globals.lo_mem_max,
        core::convert::identity,
        $lbl_bad_fmt
    );
    // undump(lo_mem_stat_max+1)(lo_mem_max)(rover);
    undump!(
        $globals,
        lo_mem_stat_max + 1,
        $globals.lo_mem_max,
        $globals.rover,
        core::convert::identity,
        $lbl_bad_fmt
    );
    // p:=mem_bot; q:=rover;
    p = mem_bot;
    q = $globals.rover;
    // repeat for k:=p to q+1 do undump_wd(mem[k]);
    loop {
        for k in p..=q + 1 {
            undump_wd!($globals, $globals.mem[k]);
        }
        // p:=q+node_size(q);
        p = q + node_size!($globals, q);
        // if (p>lo_mem_max)or((q>=rlink(q))and(rlink(q)<>rover)) then goto bad_fmt;
        if p > $globals.lo_mem_max
            || (q >= rlink!($globals, q) && rlink!($globals, q) != $globals.rover)
        {
            crate::goto_forward_label!($lbl_bad_fmt);
        }
        // q:=rlink(q);
        q = rlink!($globals, q);
        // until q=rover;
        if q == $globals.rover {
            break;
        }
    }
    // for k:=p to lo_mem_max do undump_wd(mem[k]);
    for k in p..=$globals.lo_mem_max {
        undump_wd!($globals, $globals.mem[k]);
    }
    // if mem_min<mem_bot-2 then {make more low memory available}
    if (mem_min as integer) < (mem_bot as integer) - 2 {
        /// make more low memory available
        const _: () = ();
        // begin p:=llink(rover); q:=mem_min+1;
        p = llink!($globals, $globals.rover);
        q = (mem_min + 1) as _;
        // link(mem_min):=null; info(mem_min):=null; {we don't use the bottom word}
        link!($globals, mem_min) = null;
        info_inner!($globals, mem_min) = null;
        // we don't use the bottom word
        const _: () = ();
        // rlink(p):=q; llink(rover):=q;@/
        rlink!($globals, p) = q;
        llink!($globals, $globals.rover) = q;
        // rlink(q):=rover; llink(q):=p; link(q):=empty_flag;
        rlink!($globals, q) = $globals.rover;
        llink!($globals, q) = p;
        link!($globals, q) = empty_flag;
        // node_size(q):=mem_bot-q;
        node_size!($globals, q) = mem_bot - q;
        // end;
    }
    // undump(lo_mem_max+1)(hi_mem_stat_min)(hi_mem_min);
    undump!(
        $globals,
        $globals.lo_mem_max + 1,
        hi_mem_stat_min,
        $globals.hi_mem_min,
        core::convert::identity,
        $lbl_bad_fmt
    );
    // undump(null)(mem_top)(avail); mem_end:=mem_top;
    undump!(
        $globals,
        null,
        mem_top,
        $globals.avail,
        core::convert::identity,
        $lbl_bad_fmt
    );
    $globals.mem_end = mem_top;
    // for k:=hi_mem_min to mem_end do undump_wd(mem[k]);
    for k in $globals.hi_mem_min..=$globals.mem_end {
        undump_wd!($globals, $globals.mem[k]);
    }
    // undump_int(var_used); undump_int(dyn_used)
    undump_int!($globals, $globals.var_used);
    undump_int!($globals, $globals.dyn_used);
    use crate::pascal::integer;
    use crate::section_0011::mem_min;
    use crate::section_0012::mem_bot;
    use crate::section_0012::mem_top;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0124::empty_flag;
    use crate::section_0124::llink;
    use crate::section_0124::node_size;
    use crate::section_0124::rlink;
    use crate::section_0162::hi_mem_stat_min;
    use crate::section_0162::lo_mem_stat_max;
    use crate::section_1306::undump;
    use crate::section_1306::undump_int;
    use crate::section_1306::undump_wd;
}}
