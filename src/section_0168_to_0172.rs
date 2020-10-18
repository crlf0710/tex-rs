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
