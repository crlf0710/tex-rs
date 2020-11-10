//! ` `

// @<Express astonishment...@>=
macro_rules! Express_astonishment_that_no_number_was_here {
    ($globals:expr) => {{
        // begin print_err("Missing number, treated as zero");
        print_err!($globals, strpool_str!("Missing number, treated as zero"));
        // @.Missing number...@>
        // help3("A number should have been here; I inserted `0'.")@/
        //   ("(If you can't figure out why I needed to see a number,")@/
        //   ("look up `weird error' in the index to The TeXbook.)");
        // @:TeXbook}{\sl The \TeX book@>
        // back_error;
        // end
    }}
}
