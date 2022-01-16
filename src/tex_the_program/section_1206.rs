//! @ When \.{\\halign} appears in a display, the alignment routines operate
//! essentially as they do in vertical mode. Then the following program is
//! activated, with |p| and |q| pointing to the beginning and end of the
//! resulting list, and with |aux_save| holding the |prev_depth| value.
//
// @<Finish an alignment in a display@>=
pub(crate) macro Finish_an_alignment_in_a_display($globals:expr, $p:expr, $q:expr, $aux_save:expr) {{
    // begin do_assignments;
    do_assignments($globals)?;
    // if cur_cmd<>math_shift then @<Pontificate about improper alignment in display@>
    if $globals.cur_cmd != math_shift {
        todo!("Pontificate");
    }
    // else @<Check that another \.\$ follows@>;
    else {
        crate::section_1197::Check_that_another_dollar_follows!($globals);
    }
    // pop_nest;
    pop_nest($globals);
    // tail_append(new_penalty(pre_display_penalty));
    tail_append!(
        $globals,
        new_penalty($globals, pre_display_penalty!($globals))?
    );
    // tail_append(new_param_glue(above_display_skip_code));
    tail_append!(
        $globals,
        new_param_glue($globals, above_display_skip_code.into())?
    );
    // link(tail):=p;
    link!($globals, tail!($globals)) = $p;
    // if p<>null then tail:=q;
    if $p != null {
        tail!($globals) = $q;
    }
    // tail_append(new_penalty(post_display_penalty));
    tail_append!(
        $globals,
        new_penalty($globals, post_display_penalty!($globals))?
    );
    // tail_append(new_param_glue(below_display_skip_code));
    tail_append!(
        $globals,
        new_param_glue($globals, below_display_skip_code.into())?
    );
    // prev_depth:=aux_save.sc; resume_after_display;
    prev_depth!($globals) = $aux_save[MEMORY_WORD_SC];
    resume_after_display($globals)?;
    // end
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0152::new_param_glue;
    use crate::section_0158::new_penalty;
    use crate::section_0207::math_shift;
    use crate::section_0213::prev_depth;
    use crate::section_0213::tail;
    use crate::section_0214::tail_append;
    use crate::section_0217::pop_nest;
    use crate::section_0224::above_display_skip_code;
    use crate::section_0224::below_display_skip_code;
    use crate::section_0236::post_display_penalty;
    use crate::section_0236::pre_display_penalty;
    use crate::section_1200::resume_after_display;
    use crate::section_1270::do_assignments;
}}
