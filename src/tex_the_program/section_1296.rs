//! ` `
// @<Show the current contents of a box@>=
pub(crate) macro Show_the_current_contents_of_a_box($globals:expr) {{
    // begin scan_eight_bit_int; begin_diagnostic;
    scan_eight_bit_int($globals)?;
    begin_diagnostic($globals);
    // print_nl("> \box"); print_int(cur_val); print_char("=");
    print_nl($globals, crate::strpool_str!("> \\box"));
    print_int($globals, $globals.cur_val);
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'='),
    );
    // if box(cur_val)=null then print("void")
    if r#box!($globals, $globals.cur_val) == null {
        print($globals, crate::strpool_str!("void").get() as _);
    }
    // else show_box(box(cur_val));
    else {
        show_box($globals, r#box!($globals, $globals.cur_val));
    }
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0115::null;
    use crate::section_0198::show_box;
    use crate::section_0230::r#box;
    use crate::section_0245::begin_diagnostic;
    use crate::section_0433::scan_eight_bit_int;
}}
