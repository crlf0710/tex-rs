//! @ @<Report an overfull vbox and |goto common_ending|, if...@>=
//! if (-x-total_shrink[normal]>vfuzz)or(vbadness<100) then
//!   begin print_ln; print_nl("Overfull \vbox (");
//! @.Overfull \\vbox...@>
//!   print_scaled(-x-total_shrink[normal]); print("pt too high");
//!   goto common_ending;
//!   end
//!
//! @ @<Report a tight vbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(-x,total_shrink[normal]);
//! if last_badness>vbadness then
//!   begin print_ln; print_nl("Tight \vbox (badness "); print_int(last_badness);
//! @.Tight \\vbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
