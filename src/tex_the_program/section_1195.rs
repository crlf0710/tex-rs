//! ` `

// @<Check that the necessary fonts...@>=
pub(crate) macro Check_that_the_necessary_fonts_for_math_symbols_are_present__if_not__flush_the_current_math_lists_and_set_danger_to_true($globals:expr, $danger:expr) {{
    // if (font_params[fam_fnt(2+text_size)]<total_mathsy_params)or@|
    //    (font_params[fam_fnt(2+script_size)]<total_mathsy_params)or@|
    //    (font_params[fam_fnt(2+script_script_size)]<total_mathsy_params) then
    if $globals.font_params[fam_fnt!($globals, 2 + text_size)] < total_mathsy_params
        || $globals.font_params[fam_fnt!($globals, 2 + script_size)] < total_mathsy_params
        || $globals.font_params[fam_fnt!($globals, 2 + script_script_size)] < total_mathsy_params
    {
        // begin print_err("Math formula deleted: Insufficient symbol fonts");@/
        // @.Math formula deleted...@>
        // help3("Sorry, but I can't typeset math unless \textfont 2")@/
        //   ("and \scriptfont 2 and \scriptscriptfont 2 have all")@/
        //   ("the \fontdimen values needed in math symbol fonts.");
        // error; flush_math; danger:=true;
        // end
        todo!("Math formula deleted: Insufficient symbol fonts 1");
    }
    // else if (font_params[fam_fnt(3+text_size)]<total_mathex_params)or@|
    //    (font_params[fam_fnt(3+script_size)]<total_mathex_params)or@|
    //    (font_params[fam_fnt(3+script_script_size)]<total_mathex_params) then
    else if $globals.font_params[fam_fnt!($globals, 3 + text_size)] < total_mathex_params
        || $globals.font_params[fam_fnt!($globals, 3 + script_size)] < total_mathex_params
        || $globals.font_params[fam_fnt!($globals, 3 + script_script_size)] < total_mathex_params
    {
        // begin print_err("Math formula deleted: Insufficient extension fonts");@/
        // help3("Sorry, but I can't typeset math unless \textfont 3")@/
        //   ("and \scriptfont 3 and \scriptscriptfont 3 have all")@/
        //   ("the \fontdimen values needed in math extension fonts.");
        // error; flush_math; danger:=true;
        // end
        todo!("Math formula deleted: Insufficient symbol fonts 2");
    }
    use crate::section_0230::fam_fnt;
    use crate::section_0699::*;
    use crate::section_0700::total_mathsy_params;
    use crate::section_0701::total_mathex_params;
}}
