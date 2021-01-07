//! @ @<Test if an integer is odd@>=
//! begin scan_int; b:=odd(cur_val);
//! end
//!
//! @ @<Test box register status@>=
//! begin scan_eight_bit_int; p:=box(cur_val);
//! if this_if=if_void_code then b:=(p=null)
//! else if p=null then b:=false
//! else if this_if=if_hbox_code then b:=(type(p)=hlist_node)
//! else b:=(type(p)=vlist_node);
//! end
//!
