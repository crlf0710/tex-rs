//! @ @<Fetch the |space_factor| or the |prev_depth|@>=
//! if abs(mode)<>m then
//!   begin print_err("Improper "); print_cmd_chr(set_aux,m);
//! @.Improper \\spacefactor@>
//! @.Improper \\prevdepth@>
//!   help4("You can refer to \spacefactor only in horizontal mode;")@/
//!     ("you can refer to \prevdepth only in vertical mode; and")@/
//!     ("neither of these is meaningful inside \write. So")@/
//!     ("I'm forgetting what you said and using zero instead.");
//!   error;
//!   if level<>tok_val then scanned_result(0)(dimen_val)
//!   else scanned_result(0)(int_val);
//!   end
//! else if m=vmode then scanned_result(prev_depth)(dimen_val)
//! else scanned_result(space_factor)(int_val)
//!
