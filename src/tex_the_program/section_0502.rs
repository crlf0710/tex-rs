//! ` `

// @<Display the value of |b|@>=
macro_rules! Display_the_value_of_b {
    ($globals:expr, $b:expr) => {{
        // begin begin_diagnostic;
        begin_diagnostic($globals);
        // if b then print("{true}")@+else print("{false}");
        if $b {
            print($globals, strpool_str!("{true}").get() as _);
        } else {
            print($globals, strpool_str!("{false}").get() as _);
        }
        // end_diagnostic(false);
        end_diagnostic($globals, false);
        // end

        use crate::section_0059::print;
        use crate::section_0245::begin_diagnostic;
        use crate::section_0245::end_diagnostic;
    }}
}
