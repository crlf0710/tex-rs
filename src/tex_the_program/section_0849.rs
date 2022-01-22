//! ` `

// @<Set line length parameters in preparation for hanging indentation@>=
pub(crate) macro Set_line_length_parameters_in_preparation_for_hanging_indentation($globals:expr) {{
    // begin last_special_line:=abs(hang_after);
    $globals.last_special_line = hang_after!($globals).abs() as _;
    // if hang_after<0 then
    if hang_after!($globals) < 0 {
        // begin first_width:=hsize-abs(hang_indent);
        $globals.first_width = hsize!($globals) - hang_indent!($globals).abs();
        // if hang_indent>=0 then first_indent:=hang_indent
        if hang_indent!($globals) >= scaled::zero() {
            $globals.first_indent = hang_indent!($globals);
        }
        // else first_indent:=0;
        else {
            $globals.first_indent = scaled::zero();
        }
        // second_width:=hsize; second_indent:=0;
        $globals.second_width = hsize!($globals);
        $globals.second_indent = scaled::zero();
        // end
    }
    // else  begin first_width:=hsize; first_indent:=0;
    else {
        $globals.first_width = hsize!($globals);
        $globals.first_indent = scaled::zero();
        // second_width:=hsize-abs(hang_indent);
        $globals.second_width = hsize!($globals) - hang_indent!($globals).abs();
        // if hang_indent>=0 then second_indent:=hang_indent
        if hang_indent!($globals) >= scaled::zero() {
            $globals.second_indent = hang_indent!($globals);
        }
        // else second_indent:=0;
        else {
            $globals.second_indent = scaled::zero();
        }
        // end;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0236::hang_after;
    use crate::section_0247::hang_indent;
    use crate::section_0247::hsize;
}}
