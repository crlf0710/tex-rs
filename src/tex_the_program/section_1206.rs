//! @ When \.{\\halign} appears in a display, the alignment routines operate
//! essentially as they do in vertical mode. Then the following program is
//! activated, with |p| and |q| pointing to the beginning and end of the
//! resulting list, and with |aux_save| holding the |prev_depth| value.
//!
//! @<Finish an alignment in a display@>=
//! begin do_assignments;
//! if cur_cmd<>math_shift then @<Pontificate about improper alignment in display@>
//! else @<Check that another \.\$ follows@>;
//! pop_nest;
//! tail_append(new_penalty(pre_display_penalty));
//! tail_append(new_param_glue(above_display_skip_code));
//! link(tail):=p;
//! if p<>null then tail:=q;
//! tail_append(new_penalty(post_display_penalty));
//! tail_append(new_param_glue(below_display_skip_code));
//! prev_depth:=aux_save.sc; resume_after_display;
//! end
//!
