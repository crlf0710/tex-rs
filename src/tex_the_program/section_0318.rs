//! @ But the trick is distracting us from our current goal, which is to
//! understand the input state. So let's concentrate on the data structures that
//! are being pseudoprinted as we finish up the |show_context| procedure.
//
// @<Pseudoprint the line@>=
pub(crate) macro Pseudoprint_the_line($globals:expr, $l:expr) {{
    /// end of current line in `buffer`
    let j: u8_from_0_to_n<buf_size_TYPENUM>;
    // begin_pseudoprint;
    begin_pseudoprint!($globals, $l);
    // if buffer[limit]=end_line_char then j:=limit
    if $globals.buffer[limit!($globals)].numeric_value() as integer == end_line_char!($globals) {
        j = (limit!($globals) as u8).into();
    }
    // else j:=limit+1; {determine the effective end of the line}
    else {
        j = (limit!($globals) as u8 + 1).into();
        /// determine the effective end of the line
        const _: () = ();
    }
    // if j>0 then for i:=start to j-1 do
    if j > 0 {
        for i in start!($globals)..=(j.get() as halfword - 1) {
            // begin if i=loc then set_trick_count;
            if i == loc!($globals) {
                set_trick_count!($globals);
            }
            // print(buffer[i]);
            let ch = $globals.buffer[i].numeric_value();
            if ch >= 256 {
                todo!();
            }
            print($globals, ch as _);
            // end
        }
    }
    use crate::pascal::integer;
    use crate::pascal::u8_from_0_to_n;
    use crate::section_0011::buf_size_TYPENUM;
    use crate::section_0036::loc;
    use crate::section_0059::print;
    use crate::section_0113::halfword;
    use crate::section_0236::end_line_char;
    use crate::section_0302::limit;
    use crate::section_0302::start;
    use crate::section_0316::begin_pseudoprint;
    use crate::section_0316::set_trick_count;
}}
