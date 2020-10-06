//! @* \[14] Copying boxes.
//! Another recursive operation that acts on boxes is sometimes needed: The
//! procedure |copy_node_list| returns a pointer to another node list that has
//! the same structure and meaning as the original. Note that since glue
//! specifications and token lists have reference counts, we need not make
//! copies of them. Reference counts can never get too large to fit in a
//! halfword, since each pointer to a node is in a different memory address,
//! and the total number of memory addresses fits in a halfword.
//! @^recursion@>
//! @^reference counts@>
//!
//! (Well, there actually are also references from outside |mem|; if the
//! |save_stack| is made arbitrarily large, it would theoretically be possible
//! to break \TeX\ by overflowing a reference count. But who would want to do that?)
//!
//! @d add_token_ref(#)==incr(token_ref_count(#)) {new reference to a token list}
//! @d add_glue_ref(#)==incr(glue_ref_count(#)) {new reference to a glue spec}
//!
//! @ The copying procedure copies words en masse without bothering
//! to look at their individual fields. If the node format changes---for
//! example, if the size is altered, or if some link field is moved to another
//! relative position---then this code may need to be changed too.
//! @^data structure assumptions@>
//!
//! @p function copy_node_list(@!p:pointer):pointer; {makes a duplicate of the
//!   node list that starts at |p| and returns a pointer to the new list}
//! var h:pointer; {temporary head of copied list}
//! @!q:pointer; {previous position in new list}
//! @!r:pointer; {current node being fabricated for new list}
//! @!words:0..5; {number of words remaining to be copied}
//! begin h:=get_avail; q:=h;
//! while p<>null do
//!   begin @<Make a copy of node |p| in node |r|@>;
//!   link(q):=r; q:=r; p:=link(p);
//!   end;
//! link(q):=null; q:=link(h); free_avail(h);
//! copy_node_list:=q;
//! end;
//!
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
