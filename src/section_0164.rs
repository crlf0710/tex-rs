//! ` `

// @<Initialize table entries...@>=
#[distributed_slice(INIT_TBLENTRY)]
#[allow(unused_variables)]
pub(crate) fn initialize_table_entries_done_by_initex_only_0222(globals: &mut TeXGlobals) {
    // for k:=mem_bot+1 to lo_mem_stat_max do mem[k].sc:=0;
    //   {all glue dimensions are zeroed}
    // @^data structure assumptions@>
    // k:=mem_bot;@+while k<=lo_mem_stat_max do
    //     {set first words of glue specifications}
    //   begin glue_ref_count(k):=null+1;
    //   stretch_order(k):=normal; shrink_order(k):=normal;
    //   k:=k+glue_spec_size;
    //   end;
    // stretch(fil_glue):=unity; stretch_order(fil_glue):=fil;@/
    // stretch(fill_glue):=unity; stretch_order(fill_glue):=fill;@/
    // stretch(ss_glue):=unity; stretch_order(ss_glue):=fil;@/
    // shrink(ss_glue):=unity; shrink_order(ss_glue):=fil;@/
    // stretch(fil_neg_glue):=-unity; stretch_order(fil_neg_glue):=fil;@/
    // rover:=lo_mem_stat_max+1;
    globals.rover = lo_mem_stat_max + 1;
    // link(rover):=empty_flag; {now initialize the dynamic memory}
    /// now initialize the dynamic memory
    {
        link!(globals, globals.rover) = empty_flag;
    }
    // node_size(rover):=1000; {which is a 1000-word available node}
    /// which is a 1000-word available node
    {
        node_size!(globals, globals.rover) = 1000;
    }
    // llink(rover):=rover; rlink(rover):=rover;@/
    llink!(globals, globals.rover) = globals.rover;
    rlink!(globals, globals.rover) = globals.rover;
    // lo_mem_max:=rover+1000; link(lo_mem_max):=null; info(lo_mem_max):=null;@/
    globals.lo_mem_max = globals.rover + 1000;
    link!(globals, globals.lo_mem_max) = null;
    info_inner!(globals, globals.lo_mem_max) = null;
    // for k:=hi_mem_stat_min to mem_top do
    //   mem[k]:=mem[lo_mem_max]; {clear list heads}
    // @<Initialize the special list heads and constant nodes@>;
    // avail:=null; mem_end:=mem_top;
    globals.avail = null;
    globals.mem_end = mem_top;
    // hi_mem_min:=hi_mem_stat_min; {initialize the one-word memory}
    globals.hi_mem_min = hi_mem_stat_min;
    /// initialize the one-word memory
    const _: () = ();
    // var_used:=lo_mem_stat_max+1-mem_bot; dyn_used:=hi_mem_stat_usage;
    //   {initialize statistics}
}

use crate::section_0004::TeXGlobals;
use crate::section_0008::INIT_TBLENTRY;
use crate::section_0012::mem_top;
use crate::section_0115::null;
use crate::section_0124::empty_flag;
use crate::section_0162::hi_mem_stat_min;
use crate::section_0162::lo_mem_stat_max;

use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
