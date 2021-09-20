//! @ @<Local variables for init...@>=
//! @!z:hyph_pointer; {runs through the exception dictionary}
//!
//! @ @<Set init...@>=
//! for z:=0 to hyph_size do
//!   begin hyph_word[z]:=0; hyph_list[z]:=null;
//!   end;
//! hyph_count:=0;
//!
//! @ The algorithm for exception lookup is quite simple, as soon as we have
//! a few more local variables to work with.
//!
//! @<Local variables for hyph...@>=
//! @!h:hyph_pointer; {an index into |hyph_word| and |hyph_list|}
//! @!k:str_number; {an index into |str_start|}
//! @!u:pool_pointer; {an index into |str_pool|}
//!
