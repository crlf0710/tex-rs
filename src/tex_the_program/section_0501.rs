//! ` `

// @<Either process \.{\\ifcase} or set |b|...@>=
pub(crate) macro Either_process_ifcase_or_set_b_to_the_value_of_a_boolean_condition($globals:expr, $this_if:expr, $b:expr, $save_cond_ptr:expr, $lbl_common_ending:lifetime) {{
    // case this_if of
    if false {
        unreachable!();
    }
    // if_char_code, if_cat_code: @<Test if two characters match@>;
    else if $this_if == if_char_code || $this_if == if_cat_code {
        crate::section_0506::Test_if_two_characters_match!($globals, $this_if, $b);
    }
    // if_int_code, if_dim_code: @<Test relation between integers or dimensions@>;
    else if $this_if == if_int_code || $this_if == if_dim_code {
        crate::section_0503::Test_relation_between_integers_or_dimensions!($globals, $this_if, $b);
    }
    // if_odd_code: @<Test if an integer is odd@>;
    else if $this_if == if_odd_code {
        crate::section_0504::Test_if_an_integer_is_odd!($globals, $b);
    }
    // if_vmode_code: b:=(abs(mode)=vmode);
    else if $this_if == if_vmode_code {
        $b = mode!($globals).get().abs() == vmode;
    }
    // if_hmode_code: b:=(abs(mode)=hmode);
    else if $this_if == if_hmode_code {
        $b = mode!($globals).get().abs() == hmode;
    }
    // if_mmode_code: b:=(abs(mode)=mmode);
    else if $this_if == if_mmode_code {
        $b = mode!($globals).get().abs() == mmode;
    }
    // if_inner_code: b:=(mode<0);
    else if $this_if == if_inner_code {
        $b = mode!($globals).get() < 0;
    }
    // if_void_code, if_hbox_code, if_vbox_code: @<Test box register status@>;
    else if $this_if == if_void_code || $this_if == if_hbox_code || $this_if == if_vbox_code {
        crate::section_0505::Test_box_register_status!($globals, $this_if, $b);
    }
    // ifx_code: @<Test if two tokens match@>;
    else if $this_if == ifx_code {
        crate::section_0507::Test_if_two_tokens_match!($globals, $b);
    }
    // if_eof_code: begin scan_four_bit_int; b:=(read_open[cur_val]=closed);
    else if $this_if == if_eof_code {
        scan_four_bit_int($globals)?;
        $b = $globals.read_open[$globals.cur_val as u8] == read_open_kind::closed;
        use crate::section_0435::scan_four_bit_int;
        use crate::section_0480::read_open_kind;
        // end;
    }
    // if_true_code: b:=true;
    else if $this_if == if_true_code {
        $b = true;
    }
    // if_false_code: b:=false;
    else if $this_if == if_false_code {
        $b = false;
    }
    // if_case_code: @<Select the appropriate case
    //   and |return| or |goto common_ending|@>;
    else if $this_if == if_case_code {
        crate::section_0509::Select_the_appropriate_case_and_return_or_goto_common_ending!(
            $globals,
            $save_cond_ptr,
            $lbl_common_ending
        );
    }
    // end {there are no other cases}
    else {
        crate::trace_error_expr!("this_if = {}", $this_if.get());
        /// there are no other cases
        unreachable!();
    }
    use crate::section_0211::hmode;
    use crate::section_0211::mmode;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
    use crate::section_0487::*;
}}
