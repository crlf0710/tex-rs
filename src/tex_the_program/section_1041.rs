//! @ The occurrence of blank spaces is almost part of \TeX's inner loop,
//! @^inner loop@>
//! since we usually encounter about one space for every five non-blank characters.
//! Therefore |main_control| gives second-highest priority to ordinary spaces.
//!
//! When a glue parameter like \.{\\spaceskip} is set to `\.{0pt}', we will
//! see to it later that the corresponding glue specification is precisely
//! |zero_glue|, not merely a pointer to some specification that happens
//! to be full of zeroes. Therefore it is simple to test whether a glue parameter
//! is zero or~not.
//
// @<Append a normal inter-word space...@>=
pub(crate) macro Append_a_normal_inter_word_space_to_the_current_list__then_goto_big_switch($globals:expr, $lbl_big_switch:lifetime) {{
    // if space_skip=zero_glue then
    if space_skip!($globals) == zero_glue {
        // begin @<Find the glue specification, |main_p|, for
        //   text spaces in the current font@>;
        crate::section_1042::Find_the_glue_specification_main_p_for_text_spaces_in_the_current_font!($globals);
        // temp_ptr:=new_glue(main_p);
        $globals.temp_ptr = new_glue($globals, $globals.main_p)?;
        // end
    }
    // else temp_ptr:=new_param_glue(space_skip_code);
    else {
        $globals.temp_ptr = new_param_glue($globals, small_number::new(space_skip_code as _))?;
    }
    // link(tail):=temp_ptr; tail:=temp_ptr;
    link!($globals, tail!($globals)) = $globals.temp_ptr;
    tail!($globals) = $globals.temp_ptr;
    // goto big_switch
    crate::goto_backward_label!($lbl_big_switch);
    use crate::section_0101::small_number;
    use crate::section_0118::link;
    use crate::section_0152::new_param_glue;
    use crate::section_0153::new_glue;
    use crate::section_0162::zero_glue;
    use crate::section_0213::tail;
    use crate::section_0224::space_skip;
    use crate::section_0224::space_skip_code;
}}
