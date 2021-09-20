//! @ @<Insert hyphens as specified...@>=
//! s:=hyph_list[h];
//! while s<>null do
//!   begin hyf[info(s)]:=1; s:=link(s);
//!   end
//!
//! @ @<Search |hyph_list| for pointers to |p|@>=
//! for q:=0 to hyph_size do
//!   begin if hyph_list[q]=p then
//!     begin print_nl("HYPH("); print_int(q); print_char(")");
//!     end;
//!   end
//!
