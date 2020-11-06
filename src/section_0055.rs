//! ` `

// @<Initialize the output routines@>=
macro_rules! Initialize_the_output_routines_0055 {
    ($globals:expr) => {{
        // selector:=term_only; tally:=0; term_offset:=0; file_offset:=0;
        $globals.selector = term_only.into();
        $globals.tally = 0;
        $globals.term_offset = 0.into();
        $globals.file_offset = 0.into();

        use crate::section_0054::term_only;
    }}
}
