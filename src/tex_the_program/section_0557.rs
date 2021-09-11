//! @ Here are some macros that help process ligatures and kerns.
//! We write |char_kern(f)(j)| to find the amount of kerning specified by
//! kerning command~|j| in font~|f|. If |j| is the |char_info| for a character
//! with a ligature/kern program, the first instruction of that program is either
//! |i=font_info[lig_kern_start(f)(j)]| or |font_info[lig_kern_restart(f)(i)]|,
//! depending on whether or not |skip_byte(i)<=stop_flag|.
//!
//! The constant |kern_base_offset| should be simplified, for \PASCAL\ compilers
//! that do not do local optimization.
//! @^system dependencies@>
//
// @d char_kern_end(#)==256*op_byte(#)+rem_byte(#)].sc
// @d char_kern(#)==font_info[kern_base[#]+char_kern_end
pub(crate) macro char_kern($globals:expr, $f:expr, $lk:expr) {{
    $globals.font_info[($globals.kern_base[$f] as crate::pascal::integer
        + 256 * $lk.op_byte() as crate::pascal::integer
        + $lk.rem_byte() as crate::pascal::integer)
        as crate::section_0548::font_index_repr][crate::section_0101::MEMORY_WORD_SC]
}}
// @d kern_base_offset==256*(128+min_quarterword)
pub(crate) const kern_base_offset: integer = 256 * (128 + min_quarterword as integer);
// @d lig_kern_start(#)==lig_kern_base[#]+rem_byte {beginning of lig/kern program}
/// beginning of lig/kern program
#[allow(unused_macros)]
pub(crate) macro lig_kern_start($globals:expr, $f:expr, $lk:expr) {{
    $globals.lig_kern_base[$f] + $lk.rem_byte() as crate::pascal::integer
}}
// @d lig_kern_restart_end(#)==256*op_byte(#)+rem_byte(#)+32768-kern_base_offset
// @d lig_kern_restart(#)==lig_kern_base[#]+lig_kern_restart_end
pub(crate) macro lig_kern_restart($globals:expr, $f:expr, $lk:expr) {{
    $globals.lig_kern_base[$f]
        + 256 * $lk.op_byte() as crate::pascal::integer
        + $lk.rem_byte() as crate::pascal::integer
        + 32768
        - crate::section_0557::kern_base_offset
}}

use crate::pascal::integer;
use crate::section_0110::min_quarterword;
