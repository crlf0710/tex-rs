//! @ @<Report a tight hbox and |goto common_ending|, if...@>=
//! begin last_badness:=badness(-x,total_shrink[normal]);
//! if last_badness>hbadness then
//!   begin print_ln; print_nl("Tight \hbox (badness "); print_int(last_badness);
//! @.Tight \\hbox...@>
//!   goto common_ending;
//!   end;
//! end
//!
