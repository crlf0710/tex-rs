//! @ Single-character control sequences do not need to be looked up in a hash
//! table, since we can use the character code itself as a direct address.
//! The procedure |print_cs| prints the name of a control sequence, given
//! a pointer to its address in |eqtb|. A space is printed after the name
//! unless it is a single nonletter or an active character. This procedure
//! might be invoked with invalid data, so it is ``extra robust.'' The
//! individual characters must be printed one at a time using |print|, since
//! they may be unprintable.
//
// @<Basic printing...@>=
// procedure print_cs(@!p:integer); {prints a purported control sequence}
/// prints a purported control sequence
#[allow(unused_variables)]
pub(crate) fn print_cs(globals: &mut TeXGlobals, p: integer) {
    // begin if p<hash_base then {single character}
    //   if p>=single_base then
    //     if p=null_cs then
    //       begin print_esc("csname"); print_esc("endcsname"); print_char(" ");
    //       end
    //     else  begin print_esc(p-single_base);
    //       if cat_code(p-single_base)=letter then print_char(" ");
    //       end
    //   else if p<active_base then print_esc("IMPOSSIBLE.")
    // @.IMPOSSIBLE@>
    //   else print(p-active_base)
    // else if p>=undefined_control_sequence then print_esc("IMPOSSIBLE.")
    // else if (text(p)<0)or(text(p)>=str_ptr) then print_esc("NONEXISTENT.")
    // @.NONEXISTENT@>
    // else  begin print_esc(text(p));
    //   print_char(" ");
    //   end;
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
