//! ` `

// @<Show the halfword code in |eqtb[n]|@>=
pub(crate) macro Show_the_halfword_code_in_eqtb_n($globals:expr, $n:expr) {{
    // if n<math_code_base then
    if ($n as word) < math_code_base {
        // begin if n<lc_code_base then
        if ($n as word) < lc_code_base {
            // begin print_esc("catcode"); print_int(n-cat_code_base);
            print_esc($globals, crate::strpool_str!("catcode"));
            print_int($globals, ($n as word - cat_code_base) as _);
            // end
        }
        // else if n<uc_code_base then
        else if ($n as word) < uc_code_base {
            // begin print_esc("lccode"); print_int(n-lc_code_base);
            print_esc($globals, crate::strpool_str!("lccode"));
            print_int($globals, ($n as word - lc_code_base) as _);
            // end
        }
        // else if n<sf_code_base then
        else if ($n as word) < sf_code_base {
            // begin print_esc("uccode"); print_int(n-uc_code_base);
            print_esc($globals, crate::strpool_str!("uccode"));
            print_int($globals, ($n as word - uc_code_base) as _);
            // end
        }
        // else  begin print_esc("sfcode"); print_int(n-sf_code_base);
        else {
            print_esc($globals, crate::strpool_str!("sfcode"));
            print_int($globals, ($n as word - sf_code_base) as _);
            // end;
        }
        // print_char("="); print_int(equiv(n));
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'='),
        );
        print_int($globals, equiv!($globals, $n) as _);
        // end
    }
    // else  begin print_esc("mathcode"); print_int(n-math_code_base);
    else {
        print_esc($globals, crate::strpool_str!("mathcode"));
        print_int($globals, ($n as word - math_code_base) as _);
        // print_char("="); print_int(ho(equiv(n)));
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'='),
        );
        print_int($globals, ho!(equiv!($globals, $n)) as _);
        // end
    }
    use crate::pascal::word;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0112::ho;
    use crate::section_0221::equiv;
    use crate::section_0230::cat_code_base;
    use crate::section_0230::lc_code_base;
    use crate::section_0230::math_code_base;
    use crate::section_0230::sf_code_base;
    use crate::section_0230::uc_code_base;
}}
