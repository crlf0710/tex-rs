//! ` `
// @<Complain about an undefined macro@>=
pub(crate) macro Complain_about_an_undefined_macro($globals:expr) {{
    // begin print_err("Undefined control sequence");
    print_err!($globals, crate::strpool_str!("Undefined control sequence"));
    // @.Undefined control sequence@>
    // help5("The control sequence at the end of the top line")@/
    // ("of your error message was never \def'ed. If you have")@/
    // ("misspelled it (e.g., `\hobx'), type `I' and the correct")@/
    // ("spelling (e.g., `I\hbox'). Otherwise just continue,")@/
    // ("and I'll forget about whatever was undefined.");
    help5!(
        $globals,
        crate::strpool_str!("The control sequence at the end of the top line"),
        crate::strpool_str!("of your error message was never \\def'ed. If you have"),
        crate::strpool_str!("misspelled it (e.g., `\\hobx'), type `I' and the correct"),
        crate::strpool_str!("spelling (e.g., `I\\hbox'). Otherwise just continue,"),
        crate::strpool_str!("and I'll forget about whatever was undefined.")
    );
    // error;
    error($globals)?;
    // end
    use crate::section_0073::print_err;
    use crate::section_0079::help5;
    use crate::section_0082::error;
}}
