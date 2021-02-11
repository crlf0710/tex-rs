//! @ Penalties between the lines of a paragraph come from club and widow lines,
//! from the |inter_line_penalty| parameter, and from lines that end at
//! discretionary breaks.  Breaking between lines of a two-line paragraph gets
//! both club-line and widow-line penalties. The local variable |pen| will
//! be set to the sum of all relevant penalties for the current line, except
//! that the final line is never penalized.
//
// @<Append a penalty node, if a nonzero penalty is appropriate@>=
macro_rules! Append_a_penalty_node__if_a_nonzero_penalty_is_appropriate {
    ($globals:expr, $cur_line:expr) => {{
        // if cur_line+1<>best_line then
        if $cur_line + 1 != $globals.best_line {
            // begin pen:=inter_line_penalty;
            // if cur_line=prev_graf+1 then pen:=pen+club_penalty;
            // if cur_line+2=best_line then pen:=pen+final_widow_penalty;
            // if disc_break then pen:=pen+broken_penalty;
            // if pen<>0 then
            //   begin r:=new_penalty(pen);
            //   link(tail):=r; tail:=r;
            //   end;
            // end
            todo!("append penalty node")
        }
    }}
}
