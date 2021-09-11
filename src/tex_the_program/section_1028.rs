//! ` `
// @<Ensure that box 255 is empty after output@>=
pub(crate) macro Ensure_that_box_255_is_empty_after_output($globals:expr) {{
    // if box(255)<>null then
    if r#box!($globals, 255) != null {
        // begin print_err("Output routine didn't use all of ");
        // print_esc("box"); print_int(255);
        // @.Output routine didn't use...@>
        // help3("Your \output commands should empty \box255,")@/
        //   ("e.g., by saying `\shipout\box255'.")@/
        //   ("Proceed; I'll discard its present contents.");
        // box_error(255);
        // end
        todo!("box error");
    }
    use crate::section_0115::null;
    use crate::section_0230::r#box;
}}
