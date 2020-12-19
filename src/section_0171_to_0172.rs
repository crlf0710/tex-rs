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
