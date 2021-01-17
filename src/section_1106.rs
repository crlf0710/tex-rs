//! @ @<Apologize for inability to do the operation...@>=
//! begin if (cur_chr<>glue_node)or(last_glue<>max_halfword) then
//!   begin you_cant;
//!   help2("Sorry...I usually can't take things from the current page.")@/
//!     ("Try `I\vskip-\lastskip' instead.");
//!   if cur_chr=kern_node then help_line[0]:=
//!     ("Try `I\kern-\lastkern' instead.")
//!   else if cur_chr<>glue_node then help_line[0]:=@|
//!     ("Perhaps you can make the output routine do it.");
//!   error;
//!   end;
//! end
//!
