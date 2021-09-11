//! ` `
// @<Fetch a box dimension@>=
pub(crate) macro Fetch_a_box_dimension($globals:expr, $m:expr) {{
    // begin scan_eight_bit_int;
    scan_eight_bit_int($globals)?;
    // if box(cur_val)=null then cur_val:=0 @+else cur_val:=mem[box(cur_val)+m].sc;
    if r#box!($globals, $globals.cur_val) == null {
        $globals.cur_val = 0;
    } else {
        $globals.cur_val = $globals.mem[r#box!($globals, $globals.cur_val) + $m.get() as pointer]
            [MEMORY_WORD_SC]
            .inner()
    }
    // cur_val_level:=dimen_val;
    $globals.cur_val_level = cur_val_level_kind::dimen_val;
    // end

    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0230::r#box;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0433::scan_eight_bit_int;
}}
