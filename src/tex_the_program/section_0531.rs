// @ @<Scan file name in the buffer@>=
pub(crate) macro Scan_file_name_in_the_buffer($globals:expr) {{
    // begin begin_name; k:=first;
    begin_name($globals);
    let mut k = $globals.first;
    // while (buffer[k]=" ")and(k<last) do incr(k);
    while $globals.buffer[k] == ASCII_code_literal!(b' ') && k < $globals.last {
        incr!(k);
    }
    crate::region_forward_label! {
    |'done| {
        // loop@+  begin if k=last then goto done;
        loop {
            if k == $globals.last {
                crate::goto_forward_label!('done);
            }
            // if not more_name(buffer[k]) then goto done;
            if !more_name($globals, $globals.buffer[k]) {
                crate::goto_forward_label!('done);
            }
            // incr(k);
            incr!(k);
            // end;
        }
    }
    'done <-
    // done:end_name;
    };
    end_name($globals);
    // end
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0515::begin_name;
    use crate::section_0516::more_name;
    use crate::section_0517::end_name;
}}
