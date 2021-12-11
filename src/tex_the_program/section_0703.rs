//! ` `
//!
//! When the style changes, the following piece of program computes associated
//! information:
//
// @<Set up the values of |cur_size| and |cur_mu|, based on |cur_style|@>=
pub(crate) macro Set_up_the_values_of_cur_size_and_cur_mu__based_on_cur_style($globals:expr) {{
    // begin if cur_style<script_style then cur_size:=text_size
    if $globals.cur_style.get() < style_node_subtype::script_style.get() {
        $globals.cur_size = text_size.into();
    }
    // else cur_size:=16*((cur_style-text_style) div 2);
    else {
        $globals.cur_size =
            (16 * (($globals.cur_style.get() - style_node_subtype::text_style.get()) / 2)).into();
    }
    // cur_mu:=x_over_n(math_quad(cur_size),18);
    $globals.cur_mu = x_over_n($globals, math_quad!($globals, $globals.cur_size.get()), 18);
    // end

    use crate::section_0106::x_over_n;
    use crate::section_0688::style_node_subtype;
    use crate::section_0699::text_size;
    use crate::section_0700::math_quad;
}}
