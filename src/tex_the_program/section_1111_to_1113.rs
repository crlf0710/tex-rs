//! @ @<Forbidden...@>=vmode+ital_corr,
//!
//! @ Italic corrections are converted to kern nodes when the |ital_corr| command
//! follows a character. In math mode the same effect is achieved by appending
//! a kern of zero here, since italic corrections are supplied later.
//!
//! @<Cases of |main_control| that build...@>=
//! hmode+ital_corr: append_italic_correction;
//! mmode+ital_corr: tail_append(new_kern(0));
//!
//! @ @<Declare act...@>=
//! procedure append_italic_correction;
//! label exit;
//! var p:pointer; {|char_node| at the tail of the current list}
//! @!f:internal_font_number; {the font in the |char_node|}
//! begin if tail<>head then
//!   begin if is_char_node(tail) then p:=tail
//!   else if type(tail)=ligature_node then p:=lig_char(tail)
//!   else return;
//!   f:=font(p);
//!   tail_append(new_kern(char_italic(f)(char_info(f)(character(p)))));
//!   subtype(tail):=explicit;
//!   end;
//! exit:end;
//!
