//! ` `
// @<Put the \(r)\.{\\rightskip} glue after node |q|@>=
macro_rules! Put_the_rightskip_glue_after_node_q {
    ($globals:expr, $q:expr) => {{
        /// temporary registers for list manipulation
        let r: pointer;
        // r:=new_param_glue(right_skip_code); link(r):=link(q); link(q):=r; q:=r
        r = new_param_glue($globals, right_skip_code.into())?;
        link!($globals, r) = link!($globals, $q);
        link!($globals, $q) = r;
        $q = r;
        use crate::section_0152::new_param_glue;
        use crate::section_0224::right_skip_code;
    }}
}
