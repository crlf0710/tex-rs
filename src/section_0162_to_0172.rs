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
//! @ If \TeX\ is extended improperly, the |mem| array might get screwed up.
//! For example, some pointers might be wrong, or some ``dead'' nodes might not
//! have been freed when the last reference to them disappeared. Procedures
//! |check_mem| and |search_mem| are available to help diagnose such
//! problems. These procedures make use of two arrays called |free| and
//! |was_free| that are present only if \TeX's debugging routines have
//! been included. (You may want to decrease the size of |mem| while you
//! @^debugging@>
//! are debugging.)
//!
//! @<Glob...@>=
//! @!debug @!free: packed array [mem_min..mem_max] of boolean; {free cells}
//! @t\hskip10pt@>@!was_free: packed array [mem_min..mem_max] of boolean;
//!   {previously free cells}
//! @t\hskip10pt@>@!was_mem_end,@!was_lo_max,@!was_hi_min: pointer;
//!   {previous |mem_end|, |lo_mem_max|, and |hi_mem_min|}
//! @t\hskip10pt@>@!panicking:boolean; {do we want to check memory constantly?}
//! gubed
//!
//! @ @<Set initial...@>=
//! @!debug was_mem_end:=mem_min; {indicate that everything was previously free}
//! was_lo_max:=mem_min; was_hi_min:=mem_max;
//! panicking:=false;
//! gubed
//!
//! @ Procedure |check_mem| makes sure that the available space lists of
//! |mem| are well formed, and it optionally prints out all locations
//! that are reserved now but were free the last time this procedure was called.
//!
//! @p @!debug procedure check_mem(@!print_locs : boolean);
//! label done1,done2; {loop exits}
//! var p,@!q:pointer; {current locations of interest in |mem|}
//! @!clobbered:boolean; {is something amiss?}
//! begin for p:=mem_min to lo_mem_max do free[p]:=false; {you can probably
//!   do this faster}
//! for p:=hi_mem_min to mem_end do free[p]:=false; {ditto}
//! @<Check single-word |avail| list@>;
//! @<Check variable-size |avail| list@>;
//! @<Check flags of unavailable nodes@>;
//! if print_locs then @<Print newly busy locations@>;
//! for p:=mem_min to lo_mem_max do was_free[p]:=free[p];
//! for p:=hi_mem_min to mem_end do was_free[p]:=free[p];
//!   {|was_free:=free| might be faster}
//! was_mem_end:=mem_end; was_lo_max:=lo_mem_max; was_hi_min:=hi_mem_min;
//! end;
//! gubed
//!
//! @ @<Check single-word...@>=
//! p:=avail; q:=null; clobbered:=false;
//! while p<>null do
//!   begin if (p>mem_end)or(p<hi_mem_min) then clobbered:=true
//!   else if free[p] then clobbered:=true;
//!   if clobbered then
//!     begin print_nl("AVAIL list clobbered at ");
//! @.AVAIL list clobbered...@>
//!     print_int(q); goto done1;
//!     end;
//!   free[p]:=true; q:=p; p:=link(q);
//!   end;
//! done1:
//!
//! @ @<Check variable-size...@>=
//! p:=rover; q:=null; clobbered:=false;
//! repeat if (p>=lo_mem_max)or(p<mem_min) then clobbered:=true
//!   else if (rlink(p)>=lo_mem_max)or(rlink(p)<mem_min) then clobbered:=true
//!   else if  not(is_empty(p))or(node_size(p)<2)or@|
//!    (p+node_size(p)>lo_mem_max)or@| (llink(rlink(p))<>p) then clobbered:=true;
//!   if clobbered then
//!   begin print_nl("Double-AVAIL list clobbered at ");
//!   print_int(q); goto done2;
//!   end;
//! for q:=p to p+node_size(p)-1 do {mark all locations free}
//!   begin if free[q] then
//!     begin print_nl("Doubly free location at ");
//! @.Doubly free location...@>
//!     print_int(q); goto done2;
//!     end;
//!   free[q]:=true;
//!   end;
//! q:=p; p:=rlink(p);
//! until p=rover;
//! done2:
//!
//! @ @<Check flags...@>=
//! p:=mem_min;
//! while p<=lo_mem_max do {node |p| should not be empty}
//!   begin if is_empty(p) then
//!     begin print_nl("Bad flag at "); print_int(p);
//! @.Bad flag...@>
//!     end;
//!   while (p<=lo_mem_max) and not free[p] do incr(p);
//!   while (p<=lo_mem_max) and free[p] do incr(p);
//!   end
//!
//! @ @<Print newly busy...@>=
//! begin print_nl("New busy locs:");
//! for p:=mem_min to lo_mem_max do
//!   if not free[p] and ((p>was_lo_max) or was_free[p]) then
//!     begin print_char(" "); print_int(p);
//!     end;
//! for p:=hi_mem_min to mem_end do
//!   if not free[p] and
//!    ((p<was_hi_min) or (p>was_mem_end) or was_free[p]) then
//!     begin print_char(" "); print_int(p);
//!     end;
//! end
//!
//! @ The |search_mem| procedure attempts to answer the question ``Who points
//! to node~|p|?'' In doing so, it fetches |link| and |info| fields of |mem|
//! that might not be of type |two_halves|. Strictly speaking, this is
//! @^dirty \PASCAL@>
//! undefined in \PASCAL, and it can lead to ``false drops'' (words that seem to
//! point to |p| purely by coincidence). But for debugging purposes, we want
//! to rule out the places that do {\sl not\/} point to |p|, so a few false
//! drops are tolerable.
//!
//! @p @!debug procedure search_mem(@!p:pointer); {look for pointers to |p|}
//! var q:integer; {current position being searched}
//! begin for q:=mem_min to lo_mem_max do
//!   begin if link(q)=p then
//!     begin print_nl("LINK("); print_int(q); print_char(")");
//!     end;
//!   if info(q)=p then
//!     begin print_nl("INFO("); print_int(q); print_char(")");
//!     end;
//!   end;
//! for q:=hi_mem_min to mem_end do
//!   begin if link(q)=p then
//!     begin print_nl("LINK("); print_int(q); print_char(")");
//!     end;
//!   if info(q)=p then
//!     begin print_nl("INFO("); print_int(q); print_char(")");
//!     end;
//!   end;
//! @<Search |eqtb| for equivalents equal to |p|@>;
//! @<Search |save_stack| for equivalents that point to |p|@>;
//! @<Search |hyph_list| for pointers to |p|@>;
//! end;
//! gubed
//!
