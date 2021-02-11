//! @ Now |q| points to the hlist that represents the current line of the
//! paragraph. We need to compute the appropriate line width, pack the
//! line into a box of this size, and shift the box by the appropriate
//! amount of indentation.
//
// @<Call the packaging subroutine...@>=
macro_rules! Call_the_packaging_subroutine__setting_just_box_to_the_justified_box {
    ($globals:expr) => {{
        // if cur_line>last_special_line then
        //   begin cur_width:=second_width; cur_indent:=second_indent;
        //   end
        // else if par_shape_ptr=null then
        //   begin cur_width:=first_width; cur_indent:=first_indent;
        //   end
        // else  begin cur_width:=mem[par_shape_ptr+2*cur_line].sc;
        //   cur_indent:=mem[par_shape_ptr+2*cur_line-1].sc;
        //   end;
        // adjust_tail:=adjust_head; just_box:=hpack(q,cur_width,exactly);
        // shift_amount(just_box):=cur_indent
        todo!("Call the packaging");
    }}
}