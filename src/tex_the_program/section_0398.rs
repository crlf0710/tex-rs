//! ` `
// @<Report an improper use...@>=
macro_rules! Report_an_improper_use_of_the_macro_and_abort {
    ($globals:expr) => {{
        // begin print_err("Use of "); sprint_cs(warning_index);
        print_err!($globals, strpool_str!("Use of "));
        sprint_cs($globals, $globals.warning_index);
        // @.Use of x doesn't match...@>
        // print(" doesn't match its definition");
        print($globals, strpool_str!(" doesn't match its definition").get() as _);
        // help4("If you say, e.g., `\def\a1{...}', then you must always")@/
        //   ("put `1' after `\a', since control sequence names are")@/
        //   ("made up of letters only. The macro here has not been")@/
        //   ("followed by the required stuff, so I'm ignoring it.");
        help4!($globals,
            strpool_str!("If you say, e.g., `\\def\\a1{...}', then you must always"),
            strpool_str!("put `1' after `\\a', since control sequence names are"),
            strpool_str!("made up of letters only. The macro here has not been"),
            strpool_str!("followed by the required stuff, so I'm ignoring it.")
        );
        // error; return;
        error($globals)?;
        return_nojump!();
        // end
        use crate::section_0059::print;
        use crate::section_0082::error;
        use crate::section_0263::sprint_cs;
    }}
}
