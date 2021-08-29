//! @ @<Set line length parameters in preparation for hanging indentation@>=
//! begin last_special_line:=abs(hang_after);
//! if hang_after<0 then
//!   begin first_width:=hsize-abs(hang_indent);
//!   if hang_indent>=0 then first_indent:=hang_indent
//!   else first_indent:=0;
//!   second_width:=hsize; second_indent:=0;
//!   end
//! else  begin first_width:=hsize; first_indent:=0;
//!   second_width:=hsize-abs(hang_indent);
//!   if hang_indent>=0 then second_indent:=hang_indent
//!   else second_indent:=0;
//!   end;
//! end
//!
