//! @ @<Initialize table entries...@>=
//! for k:=dimen_base to eqtb_size do eqtb[k].sc:=0;
//!
//! @ @<Show equivalent |n|, in region 6@>=
//! begin if n<scaled_base then print_length_param(n-dimen_base)
//! else  begin print_esc("dimen"); print_int(n-scaled_base);
//!   end;
//! print_char("="); print_scaled(eqtb[n].sc); print("pt");
//! end
//!
//! @ Here is a procedure that displays the contents of |eqtb[n]|
//! symbolically.
//!
//! @p@t\4@>@<Declare the procedure called |print_cmd_chr|@>@;@/
//! @!stat procedure show_eqtb(@!n:pointer);
//! begin if n<active_base then print_char("?") {this can't happen}
//! else if n<glue_base then @<Show equivalent |n|, in region 1 or 2@>
//! else if n<local_base then @<Show equivalent |n|, in region 3@>
//! else if n<int_base then @<Show equivalent |n|, in region 4@>
//! else if n<dimen_base then @<Show equivalent |n|, in region 5@>
//! else if n<=eqtb_size then @<Show equivalent |n|, in region 6@>
//! else print_char("?"); {this can't happen either}
//! end;
//! tats
//!
