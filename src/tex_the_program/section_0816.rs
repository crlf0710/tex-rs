//! @ The first task is to move the list from |head| to |temp_head| and go
//! into the enclosing semantic level. We also append the \.{\\parfillskip}
//! glue to the end of the paragraph, removing a space (or other glue node) if
//! it was there, since spaces usually precede blank lines and instances of
//! `\.{\$\$}'. The |par_fill_skip| is preceded by an infinite penalty, so
//! it will never be considered as a potential breakpoint.
//!
//! This code assumes that a |glue_node| and a |penalty_node| occupy the
//! same number of |mem|~words.
//! @^data structure assumptions@>
//
// @<Get ready to start...@>=
pub(crate) macro Get_ready_to_start_line_breaking_0816($globals:expr) {{
    // link(temp_head):=link(head);
    link!($globals, temp_head) = link!($globals, head!($globals));
    // if is_char_node(tail) then tail_append(new_penalty(inf_penalty))
    if is_char_node!($globals, tail!($globals)) {
        tail_append!($globals, new_penalty($globals, inf_penalty)?);
    }
    // else if type(tail)<>glue_node then tail_append(new_penalty(inf_penalty))
    else if r#type!($globals, tail!($globals)) != glue_node {
        tail_append!($globals, new_penalty($globals, inf_penalty)?);
    }
    // else  begin type(tail):=penalty_node; delete_glue_ref(glue_ptr(tail));
    else {
        r#type!($globals, tail!($globals)) = penalty_node;
        delete_glue_ref($globals, glue_ptr!($globals, tail!($globals)));
        // flush_node_list(leader_ptr(tail)); penalty(tail):=inf_penalty;
        flush_node_list($globals, leader_ptr!($globals, tail!($globals)))?;
        penalty!($globals, tail!($globals)) = inf_penalty;
        // end;
    }
    // link(tail):=new_param_glue(par_fill_skip_code);
    link!($globals, tail!($globals)) = new_param_glue($globals, par_fill_skip_code.into())?;
    // init_cur_lang:=prev_graf mod @'200000;
    $globals.init_cur_lang = ASCII_code::from(prev_graf!($globals) % 0o200000);
    // init_l_hyf:=prev_graf div @'20000000;
    $globals.init_l_hyf = prev_graf!($globals) / 0o20000000;
    // init_r_hyf:=(prev_graf div @'200000) mod @'100;
    $globals.init_r_hyf = (prev_graf!($globals) / 0o200000) % 0o100;
    // pop_nest;
    pop_nest($globals);
    use crate::section_0018::ASCII_code;
    use crate::section_0101::scaled;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0149::glue_node;
    use crate::section_0149::glue_ptr;
    use crate::section_0149::leader_ptr;
    use crate::section_0152::new_param_glue;
    use crate::section_0157::inf_penalty;
    use crate::section_0157::penalty;
    use crate::section_0157::penalty_node;
    use crate::section_0158::new_penalty;
    use crate::section_0162::temp_head;
    use crate::section_0201::delete_glue_ref;
    use crate::section_0202::flush_node_list;
    use crate::section_0213::head;
    use crate::section_0213::prev_graf;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0217::pop_nest;
    use crate::section_0224::par_fill_skip_code;
}}
