//! ` `
// @<Make a partial copy of the whatsit...@>=
// case subtype(p) of
// open_node: begin r:=get_node(open_node_size); words:=open_node_size;
//   end;
// write_node,special_node: begin r:=get_node(write_node_size);
//   add_token_ref(write_tokens(p)); words:=write_node_size;
//   end;
// close_node,language_node: begin r:=get_node(small_node_size);
//   words:=small_node_size;
//   end;
// othercases confusion("ext2")
// @:this can't happen ext2}{\quad ext2@>
// endcases
//