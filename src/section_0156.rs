//! @ The |new_kern| function creates a kern node having a given width.
//
// @p function new_kern(@!w:scaled):pointer;
// var p:pointer; {the new node}
// begin p:=get_node(small_node_size); type(p):=kern_node;
// subtype(p):=normal;
// width(p):=w;
// new_kern:=p;
// end;
//