//! @* \[11] Memory layout.
//! Some areas of |mem| are dedicated to fixed usage, since static allocation is
//! more efficient than dynamic allocation when we can get away with it. For
//! example, locations |mem_bot| to |mem_bot+3| are always used to store the
//! specification for glue that is `\.{0pt plus 0pt minus 0pt}'. The
//! following macro definitions accomplish the static allocation by giving
//! symbolic names to the fixed positions. Static variable-size nodes appear
//! in locations |mem_bot| through |lo_mem_stat_max|, and static single-word nodes
//! appear in locations |hi_mem_stat_min| through |mem_top|, inclusive. It is
//! harmless to let |lig_trick| and |garbage| share the same location of |mem|.
//!
//! @d zero_glue==mem_bot {specification for \.{0pt plus 0pt minus 0pt}}
//! @d fil_glue==zero_glue+glue_spec_size {\.{0pt plus 1fil minus 0pt}}
//! @d fill_glue==fil_glue+glue_spec_size {\.{0pt plus 1fill minus 0pt}}
//! @d ss_glue==fill_glue+glue_spec_size {\.{0pt plus 1fil minus 1fil}}
//! @d fil_neg_glue==ss_glue+glue_spec_size {\.{0pt plus -1fil minus 0pt}}
//! @d lo_mem_stat_max==fil_neg_glue+glue_spec_size-1 {largest statically
//!   allocated word in the variable-size |mem|}
//! @#
//! @d page_ins_head==mem_top {list of insertion data for current page}
//! @d contrib_head==mem_top-1 {vlist of items not yet on current page}
//! @d page_head==mem_top-2 {vlist for current page}
//! @d temp_head==mem_top-3 {head of a temporary list of some kind}
//! @d hold_head==mem_top-4 {head of a temporary list of another kind}
//! @d adjust_head==mem_top-5 {head of adjustment list returned by |hpack|}
//! @d active==mem_top-7 {head of active list in |line_break|, needs two words}
//! @d align_head==mem_top-8 {head of preamble list for alignments}
//! @d end_span==mem_top-9 {tail of spanned-width lists}
//! @d omit_template==mem_top-10 {a constant token list}
//! @d null_list==mem_top-11 {permanently empty list}
//! @d lig_trick==mem_top-12 {a ligature masquerading as a |char_node|}
//! @d garbage==mem_top-12 {used for scrap information}
//! @d backup_head==mem_top-13 {head of token list built by |scan_keyword|}
//! @d hi_mem_stat_min==mem_top-13 {smallest statically allocated word in
//!   the one-word |mem|}
//! @d hi_mem_stat_usage=14 {the number of one-word nodes always present}
//!
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
