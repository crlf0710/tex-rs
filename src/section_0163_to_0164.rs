//! @ The following code gets |mem| off to a good start, when \TeX\ is
//! initializing itself the slow~way.
//!
//! @<Local variables for init...@>=
//! @!k:integer; {index into |mem|, |eqtb|, etc.}
//!
//! @ @<Initialize table entries...@>=
//! for k:=mem_bot+1 to lo_mem_stat_max do mem[k].sc:=0;
//!   {all glue dimensions are zeroed}
//! @^data structure assumptions@>
//! k:=mem_bot;@+while k<=lo_mem_stat_max do
//!     {set first words of glue specifications}
//!   begin glue_ref_count(k):=null+1;
//!   stretch_order(k):=normal; shrink_order(k):=normal;
//!   k:=k+glue_spec_size;
//!   end;
//! stretch(fil_glue):=unity; stretch_order(fil_glue):=fil;@/
//! stretch(fill_glue):=unity; stretch_order(fill_glue):=fill;@/
//! stretch(ss_glue):=unity; stretch_order(ss_glue):=fil;@/
//! shrink(ss_glue):=unity; shrink_order(ss_glue):=fil;@/
//! stretch(fil_neg_glue):=-unity; stretch_order(fil_neg_glue):=fil;@/
//! rover:=lo_mem_stat_max+1;
//! link(rover):=empty_flag; {now initialize the dynamic memory}
//! node_size(rover):=1000; {which is a 1000-word available node}
//! llink(rover):=rover; rlink(rover):=rover;@/
//! lo_mem_max:=rover+1000; link(lo_mem_max):=null; info(lo_mem_max):=null;@/
//! for k:=hi_mem_stat_min to mem_top do
//!   mem[k]:=mem[lo_mem_max]; {clear list heads}
//! @<Initialize the special list heads and constant nodes@>;
//! avail:=null; mem_end:=mem_top;
//! hi_mem_min:=hi_mem_stat_min; {initialize the one-word memory}
//! var_used:=lo_mem_stat_max+1-mem_bot; dyn_used:=hi_mem_stat_usage;
//!   {initialize statistics}
//!
