//! @ @<Swap the subscript and superscript into box |x|@>=
//! begin flush_node_list(x); x:=new_noad;
//! mem[nucleus(x)]:=mem[nucleus(q)];
//! mem[supscr(x)]:=mem[supscr(q)];
//! mem[subscr(x)]:=mem[subscr(q)];@/
//! mem[supscr(q)].hh:=empty_field;
//! mem[subscr(q)].hh:=empty_field;@/
//! math_type(nucleus(q)):=sub_mlist; info(nucleus(q)):=x;
//! x:=clean_box(nucleus(q),cur_style); delta:=delta+height(x)-h; h:=height(x);
//! end
//!
