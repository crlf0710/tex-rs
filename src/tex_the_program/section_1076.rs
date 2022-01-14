//! @ The global variable |adjust_tail| will be non-null if and only if the
//! current box might include adjustments that should be appended to the
//! current vertical list.
//
// @<Append box |cur_box| to the current...@>=
pub(crate) macro Append_box_cur_box_to_the_current_list__shifted_by_box_context($globals:expr, $box_context:expr) {{
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
                /// `ord_noad` for new box in math mode
                let p;
                p = new_noad($globals)?;
                // math_type(nucleus(p)):=sub_box;
                math_type!($globals, nucleus!(p)) = math_type_kind::sub_box as _;
                // info(nucleus(p)):=cur_box; cur_box:=p;
                info_inner!($globals, nucleus!(p)) = $globals.cur_box;
                $globals.cur_box = p;
                // end;
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
    use crate::section_0115::null;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0135::shift_amount;
    use crate::section_0162::adjust_head;
    use crate::section_0211::hmode;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
    use crate::section_0213::space_factor;
    use crate::section_0213::tail;
    use crate::section_0679::append_to_vlist;
    use crate::section_0681::math_type;
    use crate::section_0681::math_type_kind;
    use crate::section_0681::nucleus;
    use crate::section_0686::new_noad;
    use crate::section_0994::build_page;
}}
