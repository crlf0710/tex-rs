//! ` `

// @<Display box |p|@>=
#[allow(unused_macros)]
pub(crate) macro Display_box_p($globals:expr, $p:expr) {{
    // begin if type(p)=hlist_node then print_esc("h")
    if r#type!($globals, $p as pointer) == hlist_node {
        print_esc($globals, crate::strpool_str!("h"));
    }
    // else if type(p)=vlist_node then print_esc("v")
    else if r#type!($globals, $p as pointer) == vlist_node {
        print_esc($globals, crate::strpool_str!("v"));
    }
    // else print_esc("unset");
    else {
        print_esc($globals, crate::strpool_str!("unset"));
    }
    // print("box("); print_scaled(height(p)); print_char("+");
    print($globals, crate::strpool_str!("box(").get() as _);
    print_scaled($globals, height!($globals, $p as pointer));
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b'+'),
    );
    // print_scaled(depth(p)); print(")x"); print_scaled(width(p));
    print_scaled($globals, depth!($globals, $p as pointer));
    print($globals, crate::strpool_str!(")x").get() as _);
    print_scaled($globals, width!($globals, $p as pointer));
    // if type(p)=unset_node then
    if r#type!($globals, $p as pointer) == unset_node {
        todo!("display unset node special");
        // @<Display special fields of the unset node |p|@>
    }
    // else  begin @<Display the value of |glue_set(p)|@>;
    else {
        crate::section_0186::Display_the_value_of_glue_set_p!($globals, $p as pointer);
        // if shift_amount(p)<>0 then
        if shift_amount!($globals, $p as pointer) != scaled::zero() {
            // begin print(", shifted "); print_scaled(shift_amount(p));
            print($globals, crate::strpool_str!(", shifted ").get() as _);
            print_scaled($globals, shift_amount!($globals, $p as pointer));
            // end;
        }
        // end;
    }
    // node_list_display(list_ptr(p)); {recursive call}
    /// recursive call
    node_list_display!($globals, list_ptr!($globals, $p as pointer));
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0101::scaled;
    use crate::section_0103::print_scaled;
    use crate::section_0115::pointer;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::hlist_node;
    use crate::section_0135::list_ptr;
    use crate::section_0135::shift_amount;
    use crate::section_0135::width;
    use crate::section_0137::vlist_node;
    use crate::section_0137::*;
    use crate::section_0159::*;
    use crate::section_0180::node_list_display;
}}
