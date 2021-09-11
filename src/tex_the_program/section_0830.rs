//! @ @<Other local variables for |try_break|@>=
//! @!prev_prev_r:pointer; {a step behind |prev_r|, if |type(prev_r)=delta_node|}
//! @!s:pointer; {runs through nodes ahead of |cur_p|}
//! @!q:pointer; {points to a new node being created}
//! @!v:pointer; {points to a glue specification or a node ahead of |cur_p|}
//! @!t:integer; {node count, if |cur_p| is a discretionary node}
//! @!f:internal_font_number; {used in character width calculation}
//! @!l:halfword; {line number of current active node}
//! @!node_r_stays_active:boolean; {should node |r| remain in the active list?}
//! @!line_width:scaled; {the current line will be justified to this width}
//! @!fit_class:very_loose_fit..tight_fit; {possible fitness class of test line}
//! @!b:halfword; {badness of test line}
//! @!d:integer; {demerits of test line}
//! @!artificial_demerits:boolean; {has |d| been forced to zero?}
//! @!save_link:pointer; {temporarily holds value of |link(cur_p)|}
//! @!shortfall:scaled; {used in badness calculations}
//!
