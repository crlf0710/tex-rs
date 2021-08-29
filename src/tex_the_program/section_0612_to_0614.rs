//! @ We might find a valid hit in a |y| or |z| byte that is already gone
//! from the buffer. But we can't change bytes that are gone forever; ``the
//! moving finger writes, $\ldots\,\,$.''
//!
//! @<Consider a node with matching width...@>=
//! case mstate+info(p) of
//! none_seen+yz_OK,none_seen+y_OK,z_seen+yz_OK,z_seen+y_OK:@t@>@;@/
//!   if location(p)<dvi_gone then goto not_found
//!   else @<Change buffered instruction to |y| or |w| and |goto found|@>;
//! none_seen+z_OK,y_seen+yz_OK,y_seen+z_OK:@t@>@;@/
//!   if location(p)<dvi_gone then goto not_found
//!   else @<Change buffered instruction to |z| or |x| and |goto found|@>;
//! none_seen+y_here,none_seen+z_here,y_seen+z_here,z_seen+y_here: goto found;
//! othercases do_nothing
//! endcases
//!
//! @ @<Change buffered instruction to |y| or |w| and |goto found|@>=
//! begin k:=location(p)-dvi_offset;
//! if k<0 then k:=k+dvi_buf_size;
//! dvi_buf[k]:=dvi_buf[k]+y1-down1;
//! info(p):=y_here; goto found;
//! end
//!
//! @ @<Change buffered instruction to |z| or |x| and |goto found|@>=
//! begin k:=location(p)-dvi_offset;
//! if k<0 then k:=k+dvi_buf_size;
//! dvi_buf[k]:=dvi_buf[k]+z1-down1;
//! info(p):=z_here; goto found;
//! end
//!
