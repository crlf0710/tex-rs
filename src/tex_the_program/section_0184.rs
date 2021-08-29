//! ` `

// @<Display box |p|@>=
#[allow(unused_macros)]
macro_rules! Display_box_p {
    ($globals:expr, $p:expr) => {{
        // begin if type(p)=hlist_node then print_esc("h")
        if r#type!($globals, $p as pointer) == hlist_node {
            print_esc($globals, strpool_str!("h"));
        }
        // else if type(p)=vlist_node then print_esc("v")
        else if r#type!($globals, $p as pointer) == vlist_node {
            print_esc($globals, strpool_str!("v"));
        }
        // else print_esc("unset");
        else {
            print_esc($globals, strpool_str!("unset"));
        }
        // print("box("); print_scaled(height(p)); print_char("+");
        print($globals, strpool_str!("box(").get() as _);
        print_scaled($globals, height!($globals, $p as pointer));
        print_char(make_globals_io_string_log_view!($globals), ASCII_code_literal!(b'+'));
        // print_scaled(depth(p)); print(")x"); print_scaled(width(p));
        print_scaled($globals, depth!($globals, $p as pointer));
        print($globals, strpool_str!(")x").get() as _);
        print_scaled($globals, width!($globals, $p as pointer));
        // if type(p)=unset_node then
        if r#type!($globals, $p as pointer) == unset_node {
            todo!("display unset node special");
            // @<Display special fields of the unset node |p|@>
        }
        // else  begin @<Display the value of |glue_set(p)|@>;
        else {
            Display_the_value_of_glue_set_p!($globals, $p as pointer);
            // if shift_amount(p)<>0 then
            if shift_amount!($globals, $p as pointer) != scaled::zero() {
                // begin print(", shifted "); print_scaled(shift_amount(p));
                print($globals, strpool_str!(", shifted ").get() as _);
                print_scaled($globals, shift_amount!($globals, $p as pointer));
                // end;
            }
            // end;
        }
        // node_list_display(list_ptr(p)); {recursive call}
        /// recursive call
        node_list_display!($globals, list_ptr!($globals, $p as pointer));
        // end
        use crate::section_0058::print_char;
        use crate::section_0063::print_esc;
        use crate::section_0101::scaled;
        use crate::section_0103::print_scaled;
    }}
}
