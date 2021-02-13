//! @ @<Report an overfull hbox and |goto common_ending|, if...@>=
//! if (-x-total_shrink[normal]>hfuzz)or(hbadness<100) then
//!   begin if (overfull_rule>0)and(-x-total_shrink[normal]>hfuzz) then
//!     begin while link(q)<>null do q:=link(q);
//!     link(q):=new_rule;
//!     width(link(q)):=overfull_rule;
//!     end;
//!   print_ln; print_nl("Overfull \hbox (");
//! @.Overfull \\hbox...@>
//!   print_scaled(-x-total_shrink[normal]); print("pt too wide");
//!   goto common_ending;
//!   end
//!
//! @ @<Report a tight hbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(-x,total_shrink[normal]);
//! if last_badness>hbadness then
//!   begin print_ln; print_nl("Tight \hbox (badness "); print_int(last_badness);
//! @.Tight \\hbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
