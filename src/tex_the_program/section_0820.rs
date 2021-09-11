//! ` `

// @<Initialize the special list heads...@>=
pub(crate) macro Initialize_the_special_list_heads_and_constant_nodes_0820($globals:expr) {{
    // type(last_active):=hyphenated; line_number(last_active):=max_halfword;
    r#type!($globals, last_active!()) = hyphenated;
    line_number!($globals, last_active!()) = max_halfword;
    // subtype(last_active):=0; {the |subtype| is never examined by the algorithm}
    /// the `subtype` is never examined by the algorithm
    const _: () = ();
    subtype!($globals, last_active!()) = 0;
    use crate::section_0110::max_halfword;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0819::hyphenated;
    use crate::section_0819::last_active;
    use crate::section_0819::line_number;
}}
