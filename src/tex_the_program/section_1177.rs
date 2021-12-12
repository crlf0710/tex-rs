//! @ @<Insert a dummy...@>=
//! begin tail_append(new_noad);
//! p:=supscr(tail)+cur_cmd-sup_mark; {|supscr| or |subscr|}
//! if t<>empty then
//!   begin if cur_cmd=sup_mark then
//!     begin print_err("Double superscript");
//! @.Double superscript@>
//!     help1("I treat `x^1^2' essentially like `x^1{}^2'.");
//!     end
//!   else  begin print_err("Double subscript");
//! @.Double subscript@>
//!     help1("I treat `x_1_2' essentially like `x_1{}_2'.");
//!     end;
//!   error;
//!   end;
//! end
//!
