//! @ @<Display leaders |p|@>=
//! begin print_esc("");
//! if subtype(p)=c_leaders then print_char("c")
//! else if subtype(p)=x_leaders then print_char("x");
//! print("leaders "); print_spec(glue_ptr(p),0);
//! node_list_display(leader_ptr(p)); {recursive call}
//! end
//!
