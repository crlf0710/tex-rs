//! ` `
// @<Cases of |handle...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1100($globals:expr) {{
    // insert_group: begin end_graf; q:=split_top_skip; add_glue_ref(q);
    let processed = if $globals.cur_group == insert_group {
        // d:=split_max_depth; f:=floating_penalty; unsave; decr(save_ptr);
        // {now |saved(0)| is the insertion number, or 255 for |vadjust|}
        // p:=vpack(link(head),natural); pop_nest;
        // if saved(0)<255 then
        //   begin tail_append(get_node(ins_node_size));
        //   type(tail):=ins_node; subtype(tail):=qi(saved(0));
        //   height(tail):=height(p)+depth(p); ins_ptr(tail):=list_ptr(p);
        //   split_top_ptr(tail):=q; depth(tail):=d; float_cost(tail):=f;
        //   end
        // else  begin tail_append(get_node(small_node_size));
        //   type(tail):=adjust_node;@/
        //   subtype(tail):=0; {the |subtype| is not used}
        //   adjust_ptr(tail):=list_ptr(p); delete_glue_ref(q);
        //   end;
        // free_node(p,box_node_size);
        // if nest_ptr=0 then build_page;
        // end;
        todo!("insert_group");
        true
    }
    // output_group: @<Resume the page builder...@>;
    else if $globals.cur_group == output_group {
        crate::section_1026::Resume_the_page_builder_after_an_output_routine_has_come_to_an_end!(
            $globals
        );
        true
    } else {
        false
    };
    use crate::section_0269::*;
    processed
}}
