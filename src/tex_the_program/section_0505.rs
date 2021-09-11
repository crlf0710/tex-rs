//! ` `

// @<Test box register status@>=
pub(crate) macro Test_box_register_status($globals:expr, $this_if:expr, $b:expr) {{
    /// for traversing token lists in `\ifx` tests
    let p: pointer;
    // begin scan_eight_bit_int; p:=box(cur_val);
    scan_eight_bit_int($globals)?;
    p = r#box!($globals, $globals.cur_val);
    // if this_if=if_void_code then b:=(p=null)
    if $this_if == if_void_code {
        $b = p == null;
    }
    // else if p=null then b:=false
    else if p == null {
        $b = false;
    }
    // else if this_if=if_hbox_code then b:=(type(p)=hlist_node)
    else if $this_if == if_hbox_code {
        $b = r#type!($globals, p) == hlist_node;
    }
    // else b:=(type(p)=vlist_node);
    else {
        $b = r#type!($globals, p) == vlist_node;
    }
    // end
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0133::r#type;
    use crate::section_0135::hlist_node;
    use crate::section_0137::vlist_node;
    use crate::section_0230::r#box;
    use crate::section_0433::scan_eight_bit_int;
    use crate::section_0487::*;
}}
