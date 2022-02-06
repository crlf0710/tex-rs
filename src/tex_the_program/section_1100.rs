//! ` `
// @<Cases of |handle...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1100($globals:expr) {{
    // insert_group: begin end_graf; q:=split_top_skip; add_glue_ref(q);
    let processed = if $globals.cur_group == insert_group {
        /// holds `split_max_depth` in `insert_group`
        let d;
        /// holds `floating_penalty` in `insert_group`
        let f;
        /// for short-term use
        let (p, q);
        end_graf($globals)?;
        q = split_top_skip!($globals);
        add_glue_ref!($globals, q);
        // d:=split_max_depth; f:=floating_penalty; unsave; decr(save_ptr);
        d = split_max_depth!($globals);
        f = floating_penalty!($globals);
        unsave($globals)?;
        decr!($globals.save_ptr);
        // {now |saved(0)| is the insertion number, or 255 for |vadjust|}
        /// now `saved(0)` is the insertion number, or 255 for `vadjust`
        const _: () = ();
        // p:=vpack(link(head),natural); pop_nest;
        p = vpack(
            $globals,
            link!($globals, head!($globals)),
            natural0!(),
            natural1!(),
        )?;
        pop_nest($globals);
        // if saved(0)<255 then
        if saved!($globals, 0) < 255 {
            // begin tail_append(get_node(ins_node_size));
            tail_append!($globals, get_node($globals, ins_node_size as _)?);
            // type(tail):=ins_node; subtype(tail):=qi(saved(0));
            r#type!($globals, tail!($globals)) = ins_node;
            subtype!($globals, tail!($globals)) = qi!(saved!($globals, 0) as quarterword);
            // height(tail):=height(p)+depth(p); ins_ptr(tail):=list_ptr(p);
            height!($globals, tail!($globals)) = height!($globals, p) + depth!($globals, p);
            ins_ptr!($globals, tail!($globals)) = list_ptr!($globals, p);
            // split_top_ptr(tail):=q; depth(tail):=d; float_cost(tail):=f;
            split_top_ptr!($globals, tail!($globals)) = q;
            depth!($globals, tail!($globals)) = d;
            float_cost!($globals, tail!($globals)) = f;
            // end
        }
        // else  begin tail_append(get_node(small_node_size));
        else {
            tail_append!($globals, get_node($globals, small_node_size as _)?);
            // type(tail):=adjust_node;@/
            r#type!($globals, tail!($globals)) = adjust_node;
            // subtype(tail):=0; {the |subtype| is not used}
            /// the `subtype` is not used
            const _: () = ();
            subtype!($globals, tail!($globals)) = 0;
            // adjust_ptr(tail):=list_ptr(p); delete_glue_ref(q);
            adjust_ptr!($globals, tail!($globals)) = list_ptr!($globals, p) as _;
            delete_glue_ref($globals, q);
            // end;
        }
        // free_node(p,box_node_size);
        free_node($globals, p, box_node_size as _);
        // if nest_ptr=0 then build_page;
        if $globals.nest_ptr == 0 {
            build_page($globals)?;
        }
        // end;
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
    use crate::section_0016::decr;
    use crate::section_0112::qi;
    use crate::section_0113::quarterword;
    use crate::section_0118::link;
    use crate::section_0125::get_node;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0135::box_node_size;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::list_ptr;
    use crate::section_0140::float_cost;
    use crate::section_0140::ins_node;
    use crate::section_0140::ins_node_size;
    use crate::section_0140::ins_ptr;
    use crate::section_0140::split_top_ptr;
    use crate::section_0141::small_node_size;
    use crate::section_0142::adjust_node;
    use crate::section_0142::adjust_ptr;
    use crate::section_0201::delete_glue_ref;
    use crate::section_0203::add_glue_ref;
    use crate::section_0213::head;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0217::pop_nest;
    use crate::section_0224::split_top_skip;
    use crate::section_0236::floating_penalty;
    use crate::section_0247::split_max_depth;
    use crate::section_0269::*;
    use crate::section_0274::saved;
    use crate::section_0281::unsave;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0668::vpack;
    use crate::section_0994::build_page;
    use crate::section_1096::end_graf;
    processed
}}
