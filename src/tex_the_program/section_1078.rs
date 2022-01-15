//! ` `

// @<Append a new leader node ...@>=
pub(crate) macro Append_a_new_leader_node_that_uses_cur_box($globals:expr, $box_context:expr) {{
    // begin @<Get the next non-blank non-relax...@>;
    crate::section_0404::Get_the_next_non_blank_non_relax_non_call_token!($globals);
    // if ((cur_cmd=hskip)and(abs(mode)<>vmode))or@|
    //    ((cur_cmd=vskip)and(abs(mode)=vmode)) then
    if ($globals.cur_cmd == hskip && mode!($globals).get().abs() != vmode)
        || ($globals.cur_cmd == vskip && mode!($globals).get().abs() == vmode)
    {
        // begin append_glue; subtype(tail):=box_context-(leader_flag-a_leaders);
        append_glue($globals)?;
        subtype!($globals, tail!($globals)) =
            ($box_context - (leader_flag - glue_node_subtype::a_leaders as integer)) as _;
        // leader_ptr(tail):=cur_box;
        leader_ptr!($globals, tail!($globals)) = $globals.cur_box;
        // end
    }
    // else  begin print_err("Leaders not followed by proper glue");
    else {
        // @.Leaders not followed by...@>
        // help3("You should say `\leaders <box or rule><hskip or vskip>'.")@/
        // ("I found the <box or rule>, but there's no suitable")@/
        // ("<hskip or vskip>, so I'm ignoring these leaders."); back_error;
        // flush_node_list(cur_box);
        // end;
        todo!("leader not followed by proper glue");
    }
    // end
    use crate::pascal::integer;
    use crate::section_0133::subtype;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0149::leader_ptr;
    use crate::section_0208::*;
    use crate::section_0211::*;
    use crate::section_0213::mode;
    use crate::section_0213::tail;
    use crate::section_1060::append_glue;
    use crate::section_1071::leader_flag;
}}
