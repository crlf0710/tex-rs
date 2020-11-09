//! @ Similarly, |delete_glue_ref| is called when a pointer to a glue
//! specification is being withdrawn.
//! @^reference counts@>
//! @d fast_delete_glue_ref(#)==@t@>@;@/
//!   begin if glue_ref_count(#)=null then free_node(#,glue_spec_size)
//!   else decr(glue_ref_count(#));
//!   end
//!
//! @p procedure delete_glue_ref(@!p:pointer); {|p| points to a glue specification}
//! fast_delete_glue_ref(p);
//!
//! @ Now we are ready to delete any node list, recursively.
//! In practice, the nodes deleted are usually charnodes (about 2/3 of the time),
//! and they are glue nodes in about half of the remaining cases.
//! @^recursion@>
//!
//! @p procedure flush_node_list(@!p:pointer); {erase list of nodes starting at |p|}
//! label done; {go here when node |p| has been freed}
//! var q:pointer; {successor to node |p|}
//! begin while p<>null do
//! @^inner loop@>
//!   begin q:=link(p);
//!   if is_char_node(p) then free_avail(p)
//!   else  begin case type(p) of
//!     hlist_node,vlist_node,unset_node: begin flush_node_list(list_ptr(p));
//!       free_node(p,box_node_size); goto done;
//!       end;
//!     rule_node: begin free_node(p,rule_node_size); goto done;
//!       end;
//!     ins_node: begin flush_node_list(ins_ptr(p));
//!       delete_glue_ref(split_top_ptr(p));
//!       free_node(p,ins_node_size); goto done;
//!       end;
//!     whatsit_node: @<Wipe out the whatsit node |p| and |goto done|@>;
//!     glue_node: begin fast_delete_glue_ref(glue_ptr(p));
//!       if leader_ptr(p)<>null then flush_node_list(leader_ptr(p));
//!       end;
//!     kern_node,math_node,penalty_node: do_nothing;
//!     ligature_node: flush_node_list(lig_ptr(p));
//!     mark_node: delete_token_ref(mark_ptr(p));
//!     disc_node: begin flush_node_list(pre_break(p));
//!       flush_node_list(post_break(p));
//!       end;
//!     adjust_node: flush_node_list(adjust_ptr(p));
//!     @t\4@>@<Cases of |flush_node_list| that arise in mlists only@>@;
//!     othercases confusion("flushing")
//! @:this can't happen flushing}{\quad flushing@>
//!     endcases;@/
//!     free_node(p,small_node_size);
//!     done:end;
//!   p:=q;
//!   end;
//! end;
//!
