//! ` `
// @<Fetch the |prev_graf|@>=
pub(crate) macro Fetch_the_prev_graf($globals:expr) {{
    // if mode=0 then scanned_result(0)(int_val) {|prev_graf=0| within \.{\\write}}
    if mode!($globals) == 0 {
        scanned_result!($globals, 0, cur_val_level_kind::int_val);
    }
    // else begin nest[nest_ptr]:=cur_list; p:=nest_ptr;
    else {
        /// index into `nest`
        let mut p;
        $globals.nest[$globals.nest_ptr] = $globals.cur_list;
        p = $globals.nest_ptr;
        // while abs(nest[p].mode_field)<>vmode do decr(p);
        while $globals.nest[p].mode_field.get().abs() != vmode {
            decr!(p);
        }
        // scanned_result(nest[p].pg_field)(int_val);
        scanned_result!(
            $globals,
            $globals.nest[p].pg_field,
            cur_val_level_kind::int_val
        );
        // end
    }
    use crate::section_0016::decr;
    use crate::section_0211::vmode;
    use crate::section_0213::mode;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0413::scanned_result;
}}
