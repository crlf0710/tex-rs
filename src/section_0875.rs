//! @ The adjustment for a desired looseness is a slightly more complicated
//! version of the loop just considered. Note that if a paragraph is broken
//! into segments by displayed equations, each segment will be subject to the
//! looseness calculation, independently of the other segments.
//
// @<Find the best active node...@>=
macro_rules! Find_the_best_active_node_for_the_desired_looseness {
    ($globals:expr) => {{
        todo!("find the best");
        // begin r:=link(active); actual_looseness:=0;
        // repeat if type(r)<>delta_node then
        //   begin line_diff:=line_number(r)-best_line;
        //   if ((line_diff<actual_looseness)and(looseness<=line_diff))or@|
        //   ((line_diff>actual_looseness)and(looseness>=line_diff)) then
        //     begin best_bet:=r; actual_looseness:=line_diff;
        //     fewest_demerits:=total_demerits(r);
        //     end
        //   else if (line_diff=actual_looseness)and@|
        //     (total_demerits(r)<fewest_demerits) then
        //     begin best_bet:=r; fewest_demerits:=total_demerits(r);
        //     end;
        //   end;
        // r:=link(r);
        // until r=last_active;
        // best_line:=line_number(best_bet);
        // end
    }}
}