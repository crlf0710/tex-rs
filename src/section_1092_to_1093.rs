//! @ @<Cases of |main_control| that build...@>=
//! hmode+start_par,mmode+start_par: indent_in_hmode;
//!
//! @ @<Declare act...@>=
//! procedure indent_in_hmode;
//! var p,@!q:pointer;
//! begin if cur_chr>0 then {\.{\\indent}}
//!   begin p:=new_null_box; width(p):=par_indent;
//!   if abs(mode)=hmode then space_factor:=1000
//!   else  begin q:=new_noad; math_type(nucleus(q)):=sub_box;
//!     info(nucleus(q)):=p; p:=q;
//!     end;
//!   tail_append(p);
//!   end;
//! end;
//!
