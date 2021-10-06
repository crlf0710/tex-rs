//! ` `

// @<Fetch a font dimension@>=
pub(crate) macro Fetch_a_font_dimension($globals:expr) {{
    // begin find_font_dimen(false); font_info[fmem_ptr].sc:=0;
    find_font_dimen($globals, false)?;
    $globals.font_info[$globals.fmem_ptr][MEMORY_WORD_SC] = scaled::zero();
    // scanned_result(font_info[cur_val].sc)(dimen_val);
    scanned_result!(
        $globals,
        $globals.font_info[$globals.cur_val as u16][MEMORY_WORD_SC].inner(),
        cur_val_level_kind::dimen_val
    );
    // end
    use crate::section_0101::scaled;
    use crate::section_0101::MEMORY_WORD_SC;
    use crate::section_0410::cur_val_level_kind;
    use crate::section_0413::scanned_result;
    use crate::section_0578::find_font_dimen;
}}
