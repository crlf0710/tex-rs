//! @ We compute the values of |easy_line| and the other local variables relating
//! to line length when the |line_break| procedure is initializing itself.
//
// @<Get ready to start...@>=
pub(crate) macro Get_ready_to_start_line_breaking_0848($globals:expr) {{
    // if par_shape_ptr=null then
    if par_shape_ptr!($globals) == null {
        // if hang_indent=0 then
        if hang_indent!($globals) == scaled::zero() {
            // begin last_special_line:=0; second_width:=hsize;
            $globals.last_special_line = 0;
            $globals.second_width = hsize!($globals);
            // second_indent:=0;
            $globals.second_indent = scaled::zero();
            // end
        }
        // else @<Set line length parameters in preparation for hanging indentation@>
        else {
            todo!("par_shape_ptr == null b");
        }
    }
    // else  begin last_special_line:=info(par_shape_ptr)-1;
    else {
        $globals.last_special_line = info_inner!($globals, par_shape_ptr!($globals)) - 1;
        todo!("get ready par_shape_ptr != null");
        // second_width:=mem[par_shape_ptr+2*(last_special_line+1)].sc;
        // second_indent:=mem[par_shape_ptr+2*last_special_line+1].sc;
        // end;
    }
    // if looseness=0 then easy_line:=last_special_line
    if looseness!($globals) == 0 {
        $globals.easy_line = $globals.last_special_line;
    }
    // else easy_line:=max_halfword
    else {
        $globals.easy_line = max_halfword;
    }
    use crate::section_0101::scaled;
    use crate::section_0110::max_halfword;
    use crate::section_0115::null;
    use crate::section_0118::info_inner;
    use crate::section_0230::par_shape_ptr;
    use crate::section_0236::looseness;
    use crate::section_0247::hang_indent;
    use crate::section_0247::hsize;
}}
