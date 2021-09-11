//! ` `

// @<Express astonishment...@>=
pub(crate) macro Express_astonishment_that_no_number_was_here($globals:expr) {{
    // begin print_err("Missing number, treated as zero");
    print_err!(
        $globals,
        crate::strpool_str!("Missing number, treated as zero")
    );
    // @.Missing number...@>
    // help3("A number should have been here; I inserted `0'.")@/
    //   ("(If you can't figure out why I needed to see a number,")@/
    //   ("look up `weird error' in the index to The TeXbook.)");
    help3!(
        $globals,
        crate::strpool_str!("A number should have been here; I inserted `0'."),
        crate::strpool_str!("(If you can't figure out why I needed to see a number,"),
        crate::strpool_str!("look up `weird error' in the index to The TeXbook.)")
    );
    // @:TeXbook}{\sl The \TeX book@>
    // back_error;
    back_error($globals)?;
    // end
    use crate::section_0073::print_err;
    use crate::section_0079::help3;
    use crate::section_0327::back_error;
}}
