//! @ Now |q| points to the hlist that represents the current line of the
//! paragraph. We need to compute the appropriate line width, pack the
//! line into a box of this size, and shift the box by the appropriate
//! amount of indentation.
//
// @<Call the packaging subroutine...@>=
macro_rules! Call_the_packaging_subroutine__setting_just_box_to_the_justified_box {
    ($globals:expr, $q:expr, $cur_line:expr) => {{
        /// width of line number `cur_line`
        let cur_width: scaled;
        /// left margin of line number `cur_line`
        let cur_indent: scaled;

        // if cur_line>last_special_line then
        if $cur_line > $globals.last_special_line {
            // begin cur_width:=second_width; cur_indent:=second_indent;
            cur_width = $globals.second_width;
            cur_indent = $globals.second_indent;
            // end
        }
        // else if par_shape_ptr=null then
        else if par_shape_ptr!($globals) == null {
            // begin cur_width:=first_width; cur_indent:=first_indent;
            cur_width = $globals.first_width;
            cur_indent = $globals.first_indent;
            // end
        }
        // else  begin cur_width:=mem[par_shape_ptr+2*cur_line].sc;
        else {
            cur_width = $globals.mem[par_shape_ptr!($globals) + 2 * $cur_line][MEMORY_WORD_SC];
            // cur_indent:=mem[par_shape_ptr+2*cur_line-1].sc;
            cur_indent = $globals.mem[par_shape_ptr!($globals) + 2 * $cur_line - 1][MEMORY_WORD_SC];
            // end;
        }
        // adjust_tail:=adjust_head; just_box:=hpack(q,cur_width,exactly);
        $globals.adjust_tail = adjust_head;
        $globals.just_box = hpack($globals, $q, cur_width, exactly.into())?;
        // shift_amount(just_box):=cur_indent
        shift_amount!($globals, $globals.just_box) = cur_indent;
        use crate::section_0101::MEMORY_WORD_SC;
        use crate::section_0101::scaled;
        use crate::section_0162::adjust_head;
        use crate::section_0644::exactly;
        use crate::section_0649::hpack;
    }}
}