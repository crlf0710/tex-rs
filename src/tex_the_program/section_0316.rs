//! @ The following code sets up the print routines so that they will gather
//! the desired information.
//
// @d begin_pseudoprint==
macro_rules! begin_pseudoprint {
    ($globals:expr, $l:expr) => {{
        // begin l:=tally; tally:=0; selector:=pseudo;
        $l = ($globals.tally as u8).into();
        $globals.tally = 0;
        $globals.selector = pseudo.into();
        // trick_count:=1000000;
        $globals.trick_count = 1000000;
        // end

        use crate::section_0054::pseudo;
    }};
}

// @d set_trick_count==
macro_rules! set_trick_count {
    ($globals:expr) => {{
        // begin first_count:=tally;
        $globals.first_count = $globals.tally;
        // trick_count:=tally+1+error_line-half_error_line;
        $globals.trick_count = $globals.tally + 1 + $globals.error_line as integer
            - $globals.half_error_line as integer;
        // if trick_count<error_line then trick_count:=error_line;
        if $globals.trick_count < $globals.error_line as _ {
            $globals.trick_count = $globals.error_line as _;
        }
        // end
    }};
}
