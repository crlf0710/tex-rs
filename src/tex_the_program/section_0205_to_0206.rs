//! @ @<Make a copy of node |p|...@>=
//! words:=1; {this setting occurs in more branches than any other}
//! if is_char_node(p) then r:=get_avail
//! else @<Case statement to copy different types and set |words| to the number
//!   of initial words not yet copied@>;
//! while words>0 do
//!   begin decr(words); mem[r+words]:=mem[p+words];
//!   end
//!
//! @ @<Case statement to copy...@>=
//! case type(p) of
//! hlist_node,vlist_node,unset_node: begin r:=get_node(box_node_size);
//!   mem[r+6]:=mem[p+6]; mem[r+5]:=mem[p+5]; {copy the last two words}
//!   list_ptr(r):=copy_node_list(list_ptr(p)); {this affects |mem[r+5]|}
//!   words:=5;
//!   end;
//! rule_node: begin r:=get_node(rule_node_size); words:=rule_node_size;
//!   end;
//! ins_node: begin r:=get_node(ins_node_size); mem[r+4]:=mem[p+4];
//!   add_glue_ref(split_top_ptr(p));
//!   ins_ptr(r):=copy_node_list(ins_ptr(p)); {this affects |mem[r+4]|}
//!   words:=ins_node_size-1;
//!   end;
//! whatsit_node:@<Make a partial copy of the whatsit node |p| and make |r|
//!   point to it; set |words| to the number of initial words not yet copied@>;
//! glue_node: begin r:=get_node(small_node_size); add_glue_ref(glue_ptr(p));
//!   glue_ptr(r):=glue_ptr(p); leader_ptr(r):=copy_node_list(leader_ptr(p));
//!   end;
//! kern_node,math_node,penalty_node: begin r:=get_node(small_node_size);
//!   words:=small_node_size;
//!   end;
//! ligature_node: begin r:=get_node(small_node_size);
//!   mem[lig_char(r)]:=mem[lig_char(p)]; {copy |font| and |character|}
//!   lig_ptr(r):=copy_node_list(lig_ptr(p));
//!   end;
//! disc_node: begin r:=get_node(small_node_size);
//!   pre_break(r):=copy_node_list(pre_break(p));
//!   post_break(r):=copy_node_list(post_break(p));
//!   end;
//! mark_node: begin r:=get_node(small_node_size); add_token_ref(mark_ptr(p));
//!   words:=small_node_size;
//!   end;
//! adjust_node: begin r:=get_node(small_node_size);
//!   adjust_ptr(r):=copy_node_list(adjust_ptr(p));
//!   end; {|words=1=small_node_size-1|}
//! othercases confusion("copying")
//! @:this can't happen copying}{\quad copying@>
//! endcases
//!
