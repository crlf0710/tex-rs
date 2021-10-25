//! @ @<Cases of |print_cmd_chr|...@>=
//! math_comp: case chr_code of
//!   ord_noad: print_esc("mathord");
//!   op_noad: print_esc("mathop");
//!   bin_noad: print_esc("mathbin");
//!   rel_noad: print_esc("mathrel");
//!   open_noad: print_esc("mathopen");
//!   close_noad: print_esc("mathclose");
//!   punct_noad: print_esc("mathpunct");
//!   inner_noad: print_esc("mathinner");
//!   under_noad: print_esc("underline");
//!   othercases print_esc("overline")
//!   endcases;
//! limit_switch: if chr_code=limits then print_esc("limits")
//!   else if chr_code=no_limits then print_esc("nolimits")
//!   else print_esc("displaylimits");
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+math_comp: begin tail_append(new_noad);
//!   type(tail):=cur_chr; scan_math(nucleus(tail));
//!   end;
//! mmode+limit_switch: math_limit_switch;
//!
//! @ @<Declare act...@>=
//! procedure math_limit_switch;
//! label exit;
//! begin if head<>tail then if type(tail)=op_noad then
//!   begin subtype(tail):=cur_chr; return;
//!   end;
//! print_err("Limit controls must follow a math operator");
//! @.Limit controls must follow...@>
//! help1("I'm ignoring this misplaced \limits or \nolimits command."); error;
//! exit:end;
//!
//! @ Delimiter fields of noads are filled in by the |scan_delimiter| routine.
//! The first parameter of this procedure is the |mem| address where the
//! delimiter is to be placed; the second tells if this delimiter follows
//! \.{\\radical} or not.
//!
//! @<Declare act...@>=
//! procedure scan_delimiter(@!p:pointer;@!r:boolean);
//! begin if r then scan_twenty_seven_bit_int
//! else  begin @<Get the next non-blank non-relax...@>;
//!   case cur_cmd of
//!   letter,other_char: cur_val:=del_code(cur_chr);
//!   delim_num: scan_twenty_seven_bit_int;
//!   othercases cur_val:=-1
//!   endcases;
//!   end;
//! if cur_val<0 then @<Report that an invalid delimiter code is being changed
//!    to null; set~|cur_val:=0|@>;
//! small_fam(p):=(cur_val div @'4000000) mod 16;
//! small_char(p):=qi((cur_val div @'10000) mod 256);
//! large_fam(p):=(cur_val div 256) mod 16;
//! large_char(p):=qi(cur_val mod 256);
//! end;
//!
//! @ @<Report that an invalid delimiter...@>=
//! begin print_err("Missing delimiter (. inserted)");
//! @.Missing delimiter...@>
//! help6("I was expecting to see something like `(' or `\{' or")@/
//!   ("`\}' here. If you typed, e.g., `{' instead of `\{', you")@/
//!   ("should probably delete the `{' by typing `1' now, so that")@/
//!   ("braces don't get unbalanced. Otherwise just proceed.")@/
//!   ("Acceptable delimiters are characters whose \delcode is")@/
//!   ("nonnegative, or you can use `\delimiter <delimiter code>'.");
//! back_error; cur_val:=0;
//! end
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+radical:math_radical;
//!
//! @ @<Declare act...@>=
//! procedure math_radical;
//! begin tail_append(get_node(radical_noad_size));
//! type(tail):=radical_noad; subtype(tail):=normal;
//! mem[nucleus(tail)].hh:=empty_field;
//! mem[subscr(tail)].hh:=empty_field;
//! mem[supscr(tail)].hh:=empty_field;
//! scan_delimiter(left_delimiter(tail),true); scan_math(nucleus(tail));
//! end;
//!
//! @ @<Cases of |main_control| that build...@>=
//! mmode+accent,mmode+math_accent:math_ac;
//!
//! @ @<Declare act...@>=
//! procedure math_ac;
//! begin if cur_cmd=accent then
//!   @<Complain that the user should have said \.{\\mathaccent}@>;
//! tail_append(get_node(accent_noad_size));
//! type(tail):=accent_noad; subtype(tail):=normal;
//! mem[nucleus(tail)].hh:=empty_field;
//! mem[subscr(tail)].hh:=empty_field;
//! mem[supscr(tail)].hh:=empty_field;
//! math_type(accent_chr(tail)):=math_char;
//! scan_fifteen_bit_int;
//! character(accent_chr(tail)):=qi(cur_val mod 256);
//! if (cur_val>=var_code)and fam_in_range then fam(accent_chr(tail)):=cur_fam
//! else fam(accent_chr(tail)):=(cur_val div 256) mod 16;
//! scan_math(nucleus(tail));
//! end;
//!
//! @ @<Complain that the user should have said \.{\\mathaccent}@>=
//! begin print_err("Please use "); print_esc("mathaccent");
//! print(" for accents in math mode");
//! @.Please use \\mathaccent...@>
//! help2("I'm changing \accent to \mathaccent here; wish me luck.")@/
//!   ("(Accents are not the same in formulas as they are in text.)");
//! error;
//! end
//!
