//! ` `
// @<Display glue |p|@>=
pub(crate) macro Display_glue_p($globals:expr, $p:expr) {{
    // if subtype(p)>=a_leaders then @<Display leaders |p|@>
    if subtype!($globals, $p as pointer) as integer >= glue_node_subtype::a_leaders as integer {
        todo!("Display leaders");
    }
    // else  begin print_esc("glue");
    else {
        print_esc($globals, crate::strpool_str!("glue"));
        // if subtype(p)<>normal then
        if subtype!($globals, $p as pointer) as integer != glue_node_subtype::normal as integer {
            // begin print_char("(");
            print_char(
                make_globals_io_string_log_view!($globals),
                ASCII_code_literal!(b'('),
            );
            // if subtype(p)<cond_math_glue then
            if (subtype!($globals, $p as pointer) as integer)
                < glue_node_subtype::cond_math_glue as integer
            {
                // print_skip_param(subtype(p)-1)
                print_skip_param($globals, subtype!($globals, $p as pointer) as integer - 1);
            }
            // else if subtype(p)=cond_math_glue then print_esc("nonscript")
            else if (subtype!($globals, $p as pointer) as integer)
                == glue_node_subtype::cond_math_glue as integer
            {
                print_esc($globals, crate::strpool_str!("nonscript"));
            }
            // else print_esc("mskip");
            else {
                print_esc($globals, crate::strpool_str!("mskip"));
            }
            // print_char(")");
            print_char(
                make_globals_io_string_log_view!($globals),
                ASCII_code_literal!(b')'),
            );
            // end;
        }
        // if subtype(p)<>cond_math_glue then
        if subtype!($globals, $p as pointer) as integer
            != glue_node_subtype::cond_math_glue as integer
        {
            // begin print_char(" ");
            print_char(
                make_globals_io_string_log_view!($globals),
                ASCII_code_literal!(b' '),
            );
            // if subtype(p)<cond_math_glue then print_spec(glue_ptr(p),0)
            if (subtype!($globals, $p as pointer) as integer)
                < glue_node_subtype::cond_math_glue as integer
            {
                print_spec(
                    $globals,
                    glue_ptr!($globals, $p as pointer) as integer,
                    str_number::zero(),
                );
            }
            // else print_spec(glue_ptr(p),"mu");
            else {
                print_spec(
                    $globals,
                    glue_ptr!($globals, $p as pointer) as integer,
                    crate::strpool_str!("mu"),
                );
            }
            // end;
        }
        // end
    }
    use crate::pascal::integer;
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0038::str_number;
    use crate::section_0058::print_char;
    use crate::section_0063::print_esc;
    use crate::section_0115::pointer;
    use crate::section_0133::subtype;
    use crate::section_0149::glue_node_subtype;
    use crate::section_0149::glue_ptr;
    use crate::section_0178::print_spec;
    use crate::section_0225::print_skip_param;
}}
