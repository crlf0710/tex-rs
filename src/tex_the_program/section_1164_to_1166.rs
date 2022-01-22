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
