//! ` `
// @<Dispense with trivial cases of void or bad boxes@>=
pub(crate) macro Dispense_with_trivial_cases_of_void_or_bad_boxes($globals:expr, $v:expr) {{
    // if v=null then
    if $v == null {
        // begin vsplit:=null; return;
        crate::return_nojump!(null);
        // end;
    }
    // if type(v)<>vlist_node then
    if r#type!($globals, $v) != vlist_node {
        // begin print_err(""); print_esc("vsplit"); print(" needs a ");
        // print_esc("vbox");
        // @:vsplit_}{\.{\\vsplit needs a \\vbox}@>
        // help2("The box you are trying to split is an \hbox.")@/
        // ("I can't split such a box, so I'll leave it alone.");
        // error; vsplit:=null; return;
        // end
        todo!("err");
    }
    use crate::section_0115::null;
    use crate::section_0133::r#type;
    use crate::section_0137::vlist_node;
}}
