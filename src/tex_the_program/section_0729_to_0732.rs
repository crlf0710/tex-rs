//! @ @<Convert \(a)a final |bin_noad| to an |ord_noad|@>=
//! if r_type=bin_noad then type(r):=ord_noad
//!
//! @ @<Cases for nodes that can appear in an mlist...@>=
//! style_node: begin cur_style:=subtype(q);
//!   @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>;
//!   goto done_with_node;
//!   end;
//! choice_node: @<Change this node to a style node followed by the correct choice,
//!    then |goto done_with_node|@>;
//! ins_node,mark_node,adjust_node,
//!   whatsit_node,penalty_node,disc_node: goto done_with_node;
//! rule_node: begin if height(q)>max_h then max_h:=height(q);
//!   if depth(q)>max_d then max_d:=depth(q); goto done_with_node;
//!   end;
//! glue_node: begin @<Convert \(m)math glue to ordinary glue@>;
//!   goto done_with_node;
//!   end;
//! kern_node: begin math_kern(q,cur_mu); goto done_with_node;
//!   end;
//!
//! @ @d choose_mlist(#)==begin p:=#(q); #(q):=null;@+end
//!
//! @<Change this node to a style node...@>=
//! begin case cur_style div 2 of
//! 0: choose_mlist(display_mlist); {|display_style=0|}
//! 1: choose_mlist(text_mlist); {|text_style=2|}
//! 2: choose_mlist(script_mlist); {|script_style=4|}
//! 3: choose_mlist(script_script_mlist); {|script_script_style=6|}
//! end; {there are no other cases}
//! flush_node_list(display_mlist(q));
//! flush_node_list(text_mlist(q));
//! flush_node_list(script_mlist(q));
//! flush_node_list(script_script_mlist(q));@/
//! type(q):=style_node; subtype(q):=cur_style; width(q):=0; depth(q):=0;
//! if p<>null then
//!   begin z:=link(q); link(q):=p;
//!   while link(p)<>null do p:=link(p);
//!   link(p):=z;
//!   end;
//! goto done_with_node;
//! end
//!
//! @ Conditional math glue (`\.{\\nonscript}') results in a |glue_node|
//! pointing to |zero_glue|, with |subtype(q)=cond_math_glue|; in such a case
//! the node following will be eliminated if it is a glue or kern node and if the
//! current size is different from |text_size|. Unconditional math glue
//! (`\.{\\muskip}') is converted to normal glue by multiplying the dimensions
//! by |cur_mu|.
//! @!@:non_script_}{\.{\\nonscript} primitive@>
//!
//! @<Convert \(m)math glue to ordinary glue@>=
//! if subtype(q)=mu_glue then
//!   begin x:=glue_ptr(q);
//!   y:=math_glue(x,cur_mu); delete_glue_ref(x); glue_ptr(q):=y;
//!   subtype(q):=normal;
//!   end
//! else if (cur_size<>text_size)and(subtype(q)=cond_math_glue) then
//!   begin p:=link(q);
//!   if p<>null then if (type(p)=glue_node)or(type(p)=kern_node) then
//!     begin link(q):=link(p); link(p):=null; flush_node_list(p);
//!     end;
//!   end
//!
