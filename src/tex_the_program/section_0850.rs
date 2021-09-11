//! @ When we come to the following code, we have just encountered the first
//! active node~|r| whose |line_number| field contains |l|. Thus we want to
//! compute the length of the $l\mskip1mu$th line of the current paragraph. Furthermore,
//! we want to set |old_l| to the last number in the class of line numbers
//! equivalent to~|l|.
//
// @<Compute the new line width@>=
pub(crate) macro Compute_the_new_line_width($globals:expr, $l:expr, $old_l:expr, $line_width:expr) {{
    // if l>easy_line then
    if $l > $globals.easy_line {
        // begin line_width:=second_width; old_l:=max_halfword-1;
        $line_width = $globals.second_width;
        $old_l = max_halfword - 1;
        // end
    }
    // else  begin old_l:=l;
    else {
        $old_l = $l;
        // if l>last_special_line then line_width:=second_width
        if $l > $globals.last_special_line {
            $line_width = $globals.second_width;
        }
        // else if par_shape_ptr=null then line_width:=first_width
        else if par_shape_ptr!($globals) == null {
            $line_width = $globals.first_width;
        }
        // else line_width:=mem[par_shape_ptr+2*l@,].sc;
        else {
            $line_width = $globals.mem[par_shape_ptr!($globals) + 2 * $l][MEMORY_WORD_SC];
        }
        // end
    }
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0110::max_halfword;
    use crate::section_0115::null;
    use crate::section_0230::par_shape_ptr;
}}
