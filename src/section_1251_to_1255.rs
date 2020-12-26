//! @ @<Cases of |print_cmd_chr|...@>=
//! hyph_data: if chr_code=1 then print_esc("patterns")
//!   else print_esc("hyphenation");
//!
//! @ @<Assignments@>=
//! hyph_data: if cur_chr=1 then
//!     begin @!init new_patterns; goto done;@;@+tini@/
//!     print_err("Patterns can be loaded only by INITEX");
//! @.Patterns can be...@>
//!     help0; error;
//!     repeat get_token; until cur_cmd=right_brace; {flush the patterns}
//!     return;
//!     end
//!   else  begin new_hyph_exceptions; goto done;
//!     end;
//!
//! @ All of \TeX's parameters are kept in |eqtb| except the font information,
//! the interaction mode, and the hyphenation tables; these are strictly global.
//!
//! @<Assignments@>=
//! assign_font_dimen: begin find_font_dimen(true); k:=cur_val;
//!   scan_optional_equals; scan_normal_dimen; font_info[k].sc:=cur_val;
//!   end;
//! assign_font_int: begin n:=cur_chr; scan_font_ident; f:=cur_val;
//!   scan_optional_equals; scan_int;
//!   if n=0 then hyphen_char[f]:=cur_val@+else skew_char[f]:=cur_val;
//!   end;
//!
//! @ @<Put each...@>=
//! primitive("hyphenchar",assign_font_int,0);
//! @!@:hyphen_char_}{\.{\\hyphenchar} primitive@>
//! primitive("skewchar",assign_font_int,1);
//! @!@:skew_char_}{\.{\\skewchar} primitive@>
//!
//! @ @<Cases of |print_cmd_chr|...@>=
//! assign_font_int: if chr_code=0 then print_esc("hyphenchar")
//!   else print_esc("skewchar");
//!
