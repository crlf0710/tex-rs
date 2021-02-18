//! @ @<Change font |dvi_f| to |f|@>=
//! begin if not font_used[f] then
//!   begin dvi_font_def(f); font_used[f]:=true;
//!   end;
//! if f<=64+font_base then dvi_out(f-font_base-1+fnt_num_0)
//! else  begin dvi_out(fnt1); dvi_out(f-font_base-1);
//!   end;
//! dvi_f:=f;
//! end
//!
