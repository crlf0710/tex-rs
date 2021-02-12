//! @ Penalties between the lines of a paragraph come from club and widow lines,
//! from the |inter_line_penalty| parameter, and from lines that end at
//! discretionary breaks.  Breaking between lines of a two-line paragraph gets
//! both club-line and widow-line penalties. The local variable |pen| will
//! be set to the sum of all relevant penalties for the current line, except
//! that the final line is never penalized.
//
// @<Append a penalty node, if a nonzero penalty is appropriate@>=
macro_rules! Append_a_penalty_node__if_a_nonzero_penalty_is_appropriate {
    ($globals:expr, $cur_line:expr, $disc_break:expr, $final_widow_penalty:expr) => {{
        // if cur_line+1<>best_line then
        if $cur_line + 1 != $globals.best_line {
            /// use when calculating penalties between lines
            let mut pen: integer;
            // begin pen:=inter_line_penalty;
            pen = inter_line_penalty!($globals);
            // if cur_line=prev_graf+1 then pen:=pen+club_penalty;
            if $cur_line as integer == prev_graf!($globals) + 1 {
                pen += club_penalty!($globals);
            }
            // if cur_line+2=best_line then pen:=pen+final_widow_penalty;
            if $cur_line + 2 == $globals.best_line {
                pen += $final_widow_penalty;
            }
            // if disc_break then pen:=pen+broken_penalty;
            if $disc_break {
                pen += broken_penalty!($globals);
            }
            // if pen<>0 then
            if pen != 0 {
                /// temporary registers for list manipulation
                let r: pointer;
                // begin r:=new_penalty(pen);
                r = new_penalty($globals, pen)?;
                // link(tail):=r; tail:=r;
                link!($globals, tail!($globals)) = r;
                tail!($globals) = r;
                // end;
            }
            // end
        }
        use crate::section_0158::new_penalty;
    }}
}
