//! ` `
//!
//! The |unsave| is done after everything else here; hence an appearance of
//! `\.{\\mathsurround}' inside of `\.{\$...\$}' affects the spacing at these
//! particular \.\$'s. This is consistent with the conventions of
//! `\.{\$\$...\$\$}', since `\.{\\abovedisplayskip}' inside a display affects the
//! space above that display.
//
// @<Finish math in text@>=
pub(crate) macro Finish_math_in_text($globals:expr, $p:expr) {{
    // begin tail_append(new_math(math_surround,before));
    tail_append!(
        $globals,
        new_math(
            $globals,
            math_surround!($globals),
            math_node_subtype::before
        )?
    );
    // cur_mlist:=p; cur_style:=text_style; mlist_penalties:=(mode>0); mlist_to_hlist;
    $globals.cur_mlist = $p;
    $globals.cur_style = small_number::new(style_node_subtype::text_style as _);
    $globals.mlist_penalties = mode!($globals) > 0;
    mlist_to_hlist($globals);
    // link(tail):=link(temp_head);
    link!($globals, tail!($globals)) = link!($globals, temp_head);
    // while link(tail)<>null do tail:=link(tail);
    while link!($globals, tail!($globals)) != null {
        tail!($globals) = link!($globals, tail!($globals));
    }
    // tail_append(new_math(math_surround,after));
    tail_append!(
        $globals,
        new_math($globals, math_surround!($globals), math_node_subtype::after)?
    );
    // space_factor:=1000; unsave;
    space_factor!($globals) = 1000;
    unsave($globals)?;
    // end

    use crate::section_0101::small_number;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0147::math_node_subtype;
    use crate::section_0147::new_math;
    use crate::section_0162::temp_head;
    use crate::section_0213::mode;
    use crate::section_0213::space_factor;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0247::math_surround;
    use crate::section_0281::unsave;
    use crate::section_0688::style_node_subtype;
    use crate::section_0726::mlist_to_hlist;
}}
