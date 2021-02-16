//! @ @<Ship box |p| out@>=
//! @<Update the values of |max_h| and |max_v|; but if the page is too large,
//!   |goto done|@>;
//! @<Initialize variables as |ship_out| begins@>;
//! page_loc:=dvi_offset+dvi_ptr;
//! dvi_out(bop);
//! for k:=0 to 9 do dvi_four(count(k));
//! dvi_four(last_bop); last_bop:=page_loc;
//! cur_v:=height(p)+v_offset; temp_ptr:=p;
//! if type(p)=vlist_node then vlist_out@+else hlist_out;
//! dvi_out(eop); incr(total_pages); cur_s:=-1;
//! done:
//!
