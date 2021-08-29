//! ` `
// @<Declare act...@>=
// procedure make_mark;
// var p:pointer; {new node}
// begin p:=scan_toks(false,true); p:=get_node(small_node_size);
// type(p):=mark_node; subtype(p):=0; {the |subtype| is not used}
// mark_ptr(p):=def_ref; link(tail):=p; tail:=p;
// end;
//
