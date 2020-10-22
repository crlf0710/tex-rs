//! @ An entire string is output by calling |print|. Note that if we are outputting
//! the single standard ASCII character \.c, we could call |print("c")|, since
//! |"c"=99| is the number of a single-character string, as explained above. But
//! |print_char("c")| is quicker, so \TeX\ goes directly to the |print_char|
//! routine when it knows that this is safe. (The present implementation
//! assumes that it is always safe to print a visible ASCII character.)
//! @^system dependencies@>
//
// @<Basic print...@>=
// procedure print(@!s:integer); {prints string |s|}
/// prints string `s`
#[allow(unused_variables)]
pub(crate) fn print(globals: &mut TeXGlobals, s: integer) {
    // label exit;
    // var j:pool_pointer; {current character code position}
    // @!nl:integer; {new-line character to restore}
    // begin if s>=str_ptr then s:="???" {this can't happen}
    // @.???@>
    // else if s<256 then
    //   if s<0 then s:="???" {can't happen}
    //   else begin if selector>pseudo then
    //       begin print_char(s); return; {internal strings are not expanded}
    //       end;
    //     if (@<Character |s| is the current new-line character@>) then
    //       if selector<pseudo then
    //         begin print_ln; return;
    //         end;
    //     nl:=new_line_char; new_line_char:=-1;
    //       {temporarily disable new-line character}
    //     j:=str_start[s];
    //     while j<str_start[s+1] do
    //       begin print_char(so(str_pool[j])); incr(j);
    //       end;
    //     new_line_char:=nl; return;
    //     end;
    // j:=str_start[s];
    // while j<str_start[s+1] do
    //   begin print_char(so(str_pool[j])); incr(j);
    //   end;
    // exit:end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
