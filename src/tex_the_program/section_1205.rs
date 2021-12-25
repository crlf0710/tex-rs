//! ` `

// @<Append the glue or equation number following the display@>=
pub(crate) macro Append_the_glue_or_equation_number_following_the_display($globals:expr, $a:expr, $e:expr, $l:expr, $t:expr, $g2:expr) {{
    // if (a<>null)and(e=0)and not l then
    if $a != null && $e == scaled::zero() && !$l {
        // begin tail_append(new_penalty(inf_penalty));
        // shift_amount(a):=s+z-width(a);
        // append_to_vlist(a);
        // g2:=0;
        $g2 = 0;
        // end;
        todo!("a<>null && e == 0 && !l");
    }
    // if t<>adjust_head then {migrating material comes after equation number}
    if $t != adjust_head {
        /// migrating material comes after equation number
        const _: () = ();
        // begin link(tail):=link(adjust_head); tail:=t;
        link!($globals, tail!($globals)) = link!($globals, adjust_head);
        tail!($globals) = $t;
        // end;
    }
    // tail_append(new_penalty(post_display_penalty));
    tail_append!($globals, new_penalty($globals, post_display_penalty!($globals))?);
    // if g2>0 then tail_append(new_param_glue(g2))
    if $g2 > 0 {
        tail_append!($globals, new_param_glue($globals, $g2.into())?);
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0152::new_param_glue;
    use crate::section_0158::new_penalty;
    use crate::section_0162::adjust_head;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0236::post_display_penalty;
}}
