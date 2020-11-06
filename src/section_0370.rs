//! ` `
// @<Complain about an undefined macro@>=
macro_rules! Complain_about_an_undefined_macro {
    ($globals:expr) => {{
        // begin print_err("Undefined control sequence");
        print_err!($globals, strpool_str!("Undefined control sequence"));
        // @.Undefined control sequence@>
        // help5("The control sequence at the end of the top line")@/
        // ("of your error message was never \def'ed. If you have")@/
        // ("misspelled it (e.g., `\hobx'), type `I' and the correct")@/
        // ("spelling (e.g., `I\hbox'). Otherwise just continue,")@/
        // ("and I'll forget about whatever was undefined.");
        // error;
        // end
    }}
}