//! ` `
// @<Ensure that box 255 is empty before output@>=
pub(crate) macro Ensure_that_box_255_is_empty_before_output($globals:expr) {{
    // if box(255)<>null then
    if r#box!($globals, 255) != null {
        // begin print_err(""); print_esc("box"); print("255 is not void");
        // @:box255}{\.{\\box255 is not void}@>
        // help2("You shouldn't use \box255 except in \output routines.")@/
        //   ("Proceed, and I'll discard its present contents.");
        // box_error(255);
        // end
        todo!("box error");
    }
    use crate::section_0115::null;
    use crate::section_0230::r#box;
}}
