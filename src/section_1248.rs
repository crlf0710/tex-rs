//! @ Paragraph shapes are set up in the obvious way.
//!
//! @<Assignments@>=
//! set_shape: begin scan_optional_equals; scan_int; n:=cur_val;
//!   if n<=0 then p:=null
//!   else  begin p:=get_node(2*n+1); info(p):=n;
//!     for j:=1 to n do
//!       begin scan_normal_dimen;
//!       mem[p+2*j-1].sc:=cur_val; {indentation}
//!       scan_normal_dimen;
//!       mem[p+2*j].sc:=cur_val; {width}
//!       end;
//!     end;
//!   define(par_shape_loc,shape_ref,p);
//!   end;
//!
