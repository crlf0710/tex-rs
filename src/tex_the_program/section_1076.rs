//! @ The global variable |adjust_tail| will be non-null if and only if the
//! current box might include adjustments that should be appended to the
//! current vertical list.
//
// @<Append box |cur_box| to the current...@>=
macro_rules! Append_box_cur_box_to_the_current_list__shifted_by_box_context {
    ($globals:expr, $box_context:expr) => {{
        // begin if cur_box<>null then
        if $globals.cur_box != null {
            // begin shift_amount(cur_box):=box_context;
            shift_amount!($globals, $globals.cur_box) = scaled::new_from_inner($box_context);
            // if abs(mode)=vmode then
            if mode!($globals).get().abs() == vmode {
                // begin append_to_vlist(cur_box);
                append_to_vlist($globals, $globals.cur_box)?;
                // if adjust_tail<>null then
                if $globals.adjust_tail != null {
                    // begin if adjust_head<>adjust_tail then
                    if adjust_head != $globals.adjust_tail {
                        // begin link(tail):=link(adjust_head); tail:=adjust_tail;
                        link!($globals, tail!($globals)) = link!($globals, adjust_head);
                        tail!($globals) = $globals.adjust_tail;
                        // end;
                    }
                    // adjust_tail:=null;
                    $globals.adjust_tail = null;
                    // end;
                }
                // if mode>0 then build_page;
                if mode!($globals).get() > 0 {
                    build_page($globals)?;
                }
                // end
            }
            // else  begin if abs(mode)=hmode then space_factor:=1000
            else {
                if mode!($globals).get().abs() == hmode {
                    space_factor!($globals) = 1000;
                }
                // else  begin p:=new_noad;
                else {
                    // math_type(nucleus(p)):=sub_box;
                    // info(nucleus(p)):=cur_box; cur_box:=p;
                    // end;
                    todo!("mmode");
                }
                // link(tail):=cur_box; tail:=cur_box;
                link!($globals, tail!($globals)) = $globals.cur_box;
                tail!($globals) = $globals.cur_box;
                // end;
            }
            // end;
        }
        // end
        use crate::section_0101::scaled;
        use crate::section_0162::adjust_head;
        use crate::section_0211::vmode;
        use crate::section_0211::hmode;
        use crate::section_0679::append_to_vlist;
        use crate::section_0994::build_page;
    }}
}
