//! @ The |print| subroutine will not print a string that is still being
//! created. The following procedure will.
//!
//! @p procedure print_current_string; {prints a yet-unmade string}
//! var j:pool_pointer; {points to current character code}
//! begin j:=str_start[str_ptr];
//! while j<pool_ptr do
//!   begin print_char(so(str_pool[j])); incr(j);
//!   end;
//! end;
//!
