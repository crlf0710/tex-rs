// @ @<Cases of |main_control| that build...@>=
macro_rules! Cases_of_main_control_that_build_boxes_and_lists_1090 {
    ($globals:expr, $abs_mode_plus_cur_cmd:expr) => {{
        // vmode+start_par: new_graf(cur_chr>0);
        if $abs_mode_plus_cur_cmd == vmode as u16 + start_par as u16 {
            new_graf($globals, !$globals.cur_chr.is_zero())?;
            use crate::section_1091::new_graf;
            true
        }
        // vmode+letter,vmode+other_char,vmode+char_num,vmode+char_given,
        //    vmode+math_shift,vmode+un_hbox,vmode+vrule,
        //    vmode+accent,vmode+discretionary,vmode+hskip,vmode+valign,
        //    vmode+ex_space,vmode+no_boundary:@t@>@;@/
        else if $abs_mode_plus_cur_cmd == vmode as u16 + letter as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + other_char as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + char_num as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + char_given as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + math_shift as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + un_hbox as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + vrule as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + accent as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + discretionary as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + hskip as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + valign as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + ex_space as u16 ||
            $abs_mode_plus_cur_cmd == vmode as u16 + no_boundary as u16 {
            // begin back_input; new_graf(true);
            back_input($globals);
            new_graf($globals, true)?;
            // end;
            use crate::section_1091::new_graf;
            true
        } else {
            false
        }
    }};
}
