//! ` `
// @<Cases of |handle...@>=
pub(crate) macro Cases_of_handle_right_brace_where_a_right_brace_triggers_a_delayed_action_1168($globals:expr) {{
    // vcenter_group: begin end_graf; unsave; save_ptr:=save_ptr-2;
    let processed = if $globals.cur_group == vcenter_group {
        /// for short-term use
        let p: pointer;
        end_graf($globals)?;
        unsave($globals)?;
        $globals.save_ptr -= 2;
        // p:=vpack(link(head),saved(1),saved(0)); pop_nest;
        p = vpack(
            $globals,
            link!($globals, head!($globals)),
            scaled::new_from_inner(saved!($globals, 1)),
            small_number::new(saved!($globals, 0) as _),
        )?;
        pop_nest($globals);
        // tail_append(new_noad); type(tail):=vcenter_noad;
        tail_append!($globals, new_noad($globals)?);
        r#type!($globals, tail!($globals)) = vcenter_noad;
        // math_type(nucleus(tail)):=sub_box; info(nucleus(tail)):=p;
        math_type!($globals, nucleus!(tail!($globals))) = sub_box as _;
        info_inner!($globals, nucleus!(tail!($globals))) = p;
        // end;
        true
    } else {
        false
    };
    use crate::section_0101::scaled;
    use crate::section_0101::small_number;
    use crate::section_0115::pointer;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0213::head;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0217::pop_nest;
    use crate::section_0269::*;
    use crate::section_0274::saved;
    use crate::section_0281::unsave;
    use crate::section_0668::vpack;
    use crate::section_0681::math_type;
    use crate::section_0681::nucleus;
    use crate::section_0681::sub_box;
    use crate::section_0686::new_noad;
    use crate::section_0687::vcenter_noad;
    use crate::section_1096::end_graf;
    processed
}}
