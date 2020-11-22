//! @ @<Declare act...@>=
//! procedure head_for_vmode;
//! begin if mode<0 then
//!   if cur_cmd<>hrule then off_save
//!   else  begin print_err("You can't use `");
//!     print_esc("hrule"); print("' here except with leaders");
//! @.You can't use \\hrule...@>
//!     help2("To put a horizontal rule in an hbox or an alignment,")@/
//!       ("you should use \leaders or \hrulefill (see The TeXbook).");
//!     error;
//!     end
//! else  begin back_input; cur_tok:=par_token; back_input; token_type:=inserted;
//!   end;
//! end;
//!