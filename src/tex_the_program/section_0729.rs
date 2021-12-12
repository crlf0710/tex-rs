//! ` `

// @<Convert \(a)a final |bin_noad| to an |ord_noad|@>=
pub(crate) macro Convert_a_final_bin_noad_to_an_ord_noad($globals:expr, $r:expr, $r_type:expr) {{
    // if r_type=bin_noad then type(r):=ord_noad
    if $r_type == bin_noad {
        r#type!($globals, $r) = ord_noad;
    }
    use crate::section_0133::r#type;
    use crate::section_0682::bin_noad;
    use crate::section_0682::ord_noad;
}}
