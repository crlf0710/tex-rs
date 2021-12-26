//! ` `
// @<Display math node |p|@>=
pub(crate) macro Display_math_node_p($globals:expr, $p:expr) {{
    // begin print_esc("math");
    print_esc($globals, crate::strpool_str!("math"));
    // if subtype(p)=before then print("on")
    if subtype!($globals, $p as pointer) == math_node_subtype::before as _ {
        print($globals, crate::strpool_str!("on").get() as _);
    }
    // else print("off");
    else {
        print($globals, crate::strpool_str!("off").get() as _);
    }
    // if width(p)<>0 then
    if width!($globals, $p as pointer) != scaled::zero() {
        // begin print(", surrounded "); print_scaled(width(p));
        print($globals, crate::strpool_str!(", surrounded ").get() as _);
        print_scaled($globals, width!($globals, $p as pointer));
        // end;
    }
    // end
    use crate::section_0059::print;
    use crate::section_0063::print_esc;
    use crate::section_0101::scaled;
    use crate::section_0103::print_scaled;
    use crate::section_0115::pointer;
    use crate::section_0133::subtype;
    use crate::section_0135::width;
    use crate::section_0147::math_node_subtype;
}}
