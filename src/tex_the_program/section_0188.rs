//! ` `
// @<Display insertion |p|@>=
pub(crate) macro Display_insertion_p($globals:expr, $p:expr) {{
    // begin print_esc("insert"); print_int(qo(subtype(p)));
    print_esc($globals, crate::strpool_str!("insert"));
    print_int($globals, qo!(subtype!($globals, $p as pointer)) as _);
    // print(", natural size "); print_scaled(height(p));
    print($globals, crate::strpool_str!(", natural size ").get() as _);
    print_scaled($globals, height!($globals, $p as pointer));
    // print("; split("); print_spec(split_top_ptr(p),0);
    print($globals, crate::strpool_str!("; split(").get() as _);
    print_spec(
        $globals,
        split_top_ptr!($globals, $p as pointer) as _,
        str_number::new(0),
    );
    // print_char(","); print_scaled(depth(p));
    print_char(
        make_globals_io_string_log_view!($globals),
        ASCII_code_literal!(b','),
    );
    print_scaled($globals, depth!($globals, $p as pointer));
    // print("); float cost "); print_int(float_cost(p));
    print($globals, crate::strpool_str!("); float cost ").get() as _);
    print_int($globals, float_cost!($globals, $p as pointer));
    // node_list_display(ins_ptr(p)); {recursive call}
    /// recursive call
    node_list_display!($globals, ins_ptr!($globals, $p as pointer));
    // end
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0038::str_number;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0103::print_scaled;
    use crate::section_0112::qo;
    use crate::section_0115::pointer;
    use crate::section_0133::subtype;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0140::float_cost;
    use crate::section_0140::ins_ptr;
    use crate::section_0140::split_top_ptr;
    use crate::section_0178::print_spec;
    use crate::section_0180::node_list_display;
}}
