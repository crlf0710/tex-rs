//! ` `
// @<Report an improper use...@>=
pub(crate) macro Report_an_improper_use_of_the_macro_and_abort($globals:expr) {{
    // begin print_err("Use of "); sprint_cs(warning_index);
    print_err!($globals, crate::strpool_str!("Use of "));
    sprint_cs($globals, $globals.warning_index);
    // @.Use of x doesn't match...@>
    // print(" doesn't match its definition");
    print(
        $globals,
        crate::strpool_str!(" doesn't match its definition").get() as _,
    );
    // help4("If you say, e.g., `\def\a1{...}', then you must always")@/
    //   ("put `1' after `\a', since control sequence names are")@/
    //   ("made up of letters only. The macro here has not been")@/
    //   ("followed by the required stuff, so I'm ignoring it.");
    help4!(
        $globals,
        crate::strpool_str!("If you say, e.g., `\\def\\a1{...}', then you must always"),
        crate::strpool_str!("put `1' after `\\a', since control sequence names are"),
        crate::strpool_str!("made up of letters only. The macro here has not been"),
        crate::strpool_str!("followed by the required stuff, so I'm ignoring it.")
    );
    // error; return;
    error($globals)?;
    crate::return_nojump!();
    // end
    use crate::section_0059::print;
    use crate::section_0073::print_err;
    use crate::section_0079::help4;
    use crate::section_0082::error;
    use crate::section_0263::sprint_cs;
}}
