//! ` `

macro_rules! Initialize_the_special_list_heads_and_constant_nodes {
    ($globals:expr) => {{
        Initialize_the_special_list_heads_and_constant_nodes_0797!($globals);
        Initialize_the_special_list_heads_and_constant_nodes_0820!($globals);
        Initialize_the_special_list_heads_and_constant_nodes_0981!($globals);
        Initialize_the_special_list_heads_and_constant_nodes_0988!($globals);
    }}
}

// @<Initialize table entries...@>=
#[distributed_slice(INIT_TBLENTRY)]
#[allow(unused_variables)]
pub(crate) fn initialize_table_entries_done_by_initex_only_0222(globals: &mut TeXGlobals) {
    // for k:=mem_bot+1 to lo_mem_stat_max do mem[k].sc:=0;
    //   {all glue dimensions are zeroed}
    for k in mem_bot + 1 ..= lo_mem_stat_max {
        /// all glue dimensions are zeroed
        const _ : () = ();
        globals.mem[k][MEMORY_WORD_SC] = scaled::zero();
        use crate::section_0101::MEMORY_WORD_SC;
    }
    // @^data structure assumptions@>
    // k:=mem_bot;@+while k<=lo_mem_stat_max do
    //     {set first words of glue specifications}
    for k in (mem_bot..=lo_mem_stat_max).step_by(glue_spec_size as _) {
        /// set first words of glue specifications
        const _ : () = ();
        // begin glue_ref_count(k):=null+1;
        glue_ref_count!(globals, k) = null + 1;
        // stretch_order(k):=normal; shrink_order(k):=normal;
        stretch_order!(globals, k) = glue_ord::normal as _;
        shrink_order!(globals, k) = glue_ord::normal as _;
        // k:=k+glue_spec_size;
        const _ : () = ();
        // end;
    }
    // stretch(fil_glue):=unity; stretch_order(fil_glue):=fil;@/
    stretch!(globals, fil_glue) = unity;
    stretch_order!(globals, fil_glue) = glue_ord::fil as _;
    // stretch(fill_glue):=unity; stretch_order(fill_glue):=fill;@/
    stretch!(globals, fill_glue) = unity;
    stretch_order!(globals, fill_glue) = glue_ord::fill as _;
    // stretch(ss_glue):=unity; stretch_order(ss_glue):=fil;@/
    stretch!(globals, ss_glue) = unity;
    stretch_order!(globals, ss_glue) = glue_ord::fil as _;
    // shrink(ss_glue):=unity; shrink_order(ss_glue):=fil;@/
    shrink!(globals, ss_glue) = unity;
    shrink_order!(globals, ss_glue) = glue_ord::fil as _;
    // stretch(fil_neg_glue):=-unity; stretch_order(fil_neg_glue):=fil;@/
    stretch!(globals, fil_neg_glue) = -unity;
    stretch_order!(globals, fil_neg_glue) = glue_ord::fil as _;
    // rover:=lo_mem_stat_max+1;
    globals.rover = lo_mem_stat_max + 1;
    // link(rover):=empty_flag; {now initialize the dynamic memory}
    /// now initialize the dynamic memory
    const _ : () = ();
    link!(globals, globals.rover) = empty_flag;
    // node_size(rover):=1000; {which is a 1000-word available node}
    /// which is a 1000-word available node
    const _ : () = ();
    node_size!(globals, globals.rover) = 1000;
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
    Initialize_the_special_list_heads_and_constant_nodes!(globals);
    // avail:=null; mem_end:=mem_top;
    globals.avail = null;
    globals.mem_end = mem_top;
    // hi_mem_min:=hi_mem_stat_min; {initialize the one-word memory}
    /// initialize the one-word memory
    const _: () = ();
    globals.hi_mem_min = hi_mem_stat_min;
    // var_used:=lo_mem_stat_max+1-mem_bot; dyn_used:=hi_mem_stat_usage;
    //   {initialize statistics}
    /// initialize statistics
    const _: () = ();
    globals.var_used = lo_mem_stat_max as integer + 1 - mem_bot as integer;
    globals.dyn_used = hi_mem_stat_usage as _;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0008::INIT_TBLENTRY;
use crate::section_0012::mem_top;
use crate::section_0012::mem_bot;
use crate::section_0101::scaled;
use crate::section_0101::unity;
use crate::section_0115::null;
use crate::section_0124::empty_flag;
use crate::section_0150::glue_ord;
use crate::section_0150::glue_spec_size;
use crate::section_0162::hi_mem_stat_min;
use crate::section_0162::lo_mem_stat_max;
use crate::section_0162::hi_mem_stat_usage;
use crate::section_0162::fil_glue;
use crate::section_0162::fill_glue;
use crate::section_0162::ss_glue;
use crate::section_0162::fil_neg_glue;

use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
