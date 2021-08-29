//! ` `
// @<Scan the font size specification@>=
macro_rules! Scan_the_font_size_specification {
    ($globals:expr, $s:expr) => {{
        // name_in_progress:=true; {this keeps |cur_name| from being changed}
        /// this keeps `cur_name` from being changed
        const _ : () = ();
        $globals.name_in_progress = true;
        // if scan_keyword("at") then @<Put the \(p)(positive) `at' size into |s|@>
        if scan_keyword($globals, strpool_str!("at"))? {
            Put_the_positive_at_size_into_s!($globals, $s);
            // @.at@>
        }
        // else if scan_keyword("scaled") then
        else if scan_keyword($globals, strpool_str!("scaled"))? {
            // @.scaled@>
            // begin scan_int; s:=-cur_val;
            scan_int($globals)?;
            $s = scaled::new_from_inner(-$globals.cur_val);
            // if (cur_val<=0)or(cur_val>32768) then
            if $globals.cur_val <= 0 || $globals.cur_val > 32768 {
                // begin print_err("Illegal magnification has been changed to 1000");@/
                print_err!($globals, strpool_str!("Illegal magnification has been changed to 1000"));
                // @.Illegal magnification...@>
                // help1("The magnification ratio must be between 1 and 32768.");
                help1!($globals, strpool_str!("The magnification ratio must be between 1 and 32768."));
                // int_error(cur_val); s:=-1000;
                int_error($globals, $globals.cur_val)?;
                $s = scaled::new_from_inner(-1000);
                // end;
            }
            // end
        }
        // else s:=-1000;
        else {
            $s = scaled::new_from_inner(-1000);
        }
        // name_in_progress:=false
        $globals.name_in_progress = false;
        use crate::section_0091::int_error;
        use crate::section_0407::scan_keyword;
        use crate::section_0440::scan_int;  
    }}
}
