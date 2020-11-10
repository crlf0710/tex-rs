//! @ @<Declare act...@>=
//! procedure head_for_vmode;
//! begin if mode<0 then
//!   if cur_cmd<>hrule then off_save
//!   else  begin print_err("You can't use `");
//!     print_esc("hrule"); print("' here except with leaders");
//! @.You can't use \\hrule...@>
//!     help2("To put a horizontal rule in an hbox or an alignment,")@/
//!       ("you should use \leaders or \hrulefill (see The TeXbook).");
//!     error;
//!     end
//! else  begin back_input; cur_tok:=par_token; back_input; token_type:=inserted;
//!   end;
//! end;
//!
//! @ @<Declare act...@>=
//! procedure end_graf;
//! begin if mode=hmode then
//!   begin if head=tail then pop_nest {null paragraphs are ignored}
//!   else line_break(widow_penalty);
//!   normal_paragraph;
//!   error_count:=0;
//!   end;
//! end;
//!
//! @ Insertion and adjustment and mark nodes are constructed by the following
//! pieces of the program.
//!
//! @<Cases of |main_control| that build...@>=
//! any_mode(insert),hmode+vadjust,mmode+vadjust: begin_insert_or_adjust;
//! any_mode(mark): make_mark;
//!
//! @ @<Forbidden...@>=
//! vmode+vadjust,
//!
//! @ @<Declare act...@>=
//! procedure begin_insert_or_adjust;
//! begin if cur_cmd=vadjust then cur_val:=255
//! else  begin scan_eight_bit_int;
//!   if cur_val=255 then
//!     begin print_err("You can't "); print_esc("insert"); print_int(255);
//! @.You can't \\insert255@>
//!     help1("I'm changing to \insert0; box 255 is special.");
//!     error; cur_val:=0;
//!     end;
//!   end;
//! saved(0):=cur_val; incr(save_ptr);
//! new_save_level(insert_group); scan_left_brace; normal_paragraph;
//! push_nest; mode:=-vmode; prev_depth:=ignore_depth;
//! end;
//!
//! @ @<Cases of |handle...@>=
//! insert_group: begin end_graf; q:=split_top_skip; add_glue_ref(q);
//!   d:=split_max_depth; f:=floating_penalty; unsave; decr(save_ptr);
//!   {now |saved(0)| is the insertion number, or 255 for |vadjust|}
//!   p:=vpack(link(head),natural); pop_nest;
//!   if saved(0)<255 then
//!     begin tail_append(get_node(ins_node_size));
//!     type(tail):=ins_node; subtype(tail):=qi(saved(0));
//!     height(tail):=height(p)+depth(p); ins_ptr(tail):=list_ptr(p);
//!     split_top_ptr(tail):=q; depth(tail):=d; float_cost(tail):=f;
//!     end
//!   else  begin tail_append(get_node(small_node_size));
//!     type(tail):=adjust_node;@/
//!     subtype(tail):=0; {the |subtype| is not used}
//!     adjust_ptr(tail):=list_ptr(p); delete_glue_ref(q);
//!     end;
//!   free_node(p,box_node_size);
//!   if nest_ptr=0 then build_page;
//!   end;
//! output_group: @<Resume the page builder...@>;
//!
//! @ @<Declare act...@>=
//! procedure make_mark;
//! var p:pointer; {new node}
//! begin p:=scan_toks(false,true); p:=get_node(small_node_size);
//! type(p):=mark_node; subtype(p):=0; {the |subtype| is not used}
//! mark_ptr(p):=def_ref; link(tail):=p; tail:=p;
//! end;
//!
