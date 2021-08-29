//! ` `

// @<Put the \(p)(positive) `at' size into |s|@>=
macro_rules! Put_the_positive_at_size_into_s {
    ($globals:expr, $s:expr) => {{
        // begin scan_normal_dimen; s:=cur_val;
        scan_normal_dimen!($globals)?;
        $s = scaled::new_from_inner($globals.cur_val);
        // if (s<=0)or(s>=@'1000000000) then
        if $s.inner() <= 0 || $s.inner() >= 0o1000000000 {
            // begin print_err("Improper `at' size (");
            print_err!($globals, strpool_str!("Improper `at' size ("));
            // print_scaled(s); print("pt), replaced by 10pt");
            print_scaled($globals, $s);
            print($globals, strpool_str!("pt), replaced by 10pt").get() as _);
            // @.Improper `at' size...@>
            // help2("I can only handle fonts at positive sizes that are")@/
            // ("less than 2048pt, so I've changed what you said to 10pt.");
            help2!(
                $globals,
                strpool_str!("I can only handle fonts at positive sizes that are"),
                strpool_str!("less than 2048pt, so I've changed what you said to 10pt.")
            );
            // error; s:=10*unity;
            error($globals)?;
            $s = scaled::new_from_inner(10 * unity.inner());
            // end;
        }
        // end
        use crate::section_0082::error;
        use crate::section_0101::unity;
        use crate::section_0103::print_scaled;
    }};
}
