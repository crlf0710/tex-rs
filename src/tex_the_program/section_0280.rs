//! @ Subroutine |save_for_after| puts a token on the stack for save-keeping.
//
// @p procedure save_for_after(@!t:halfword);
pub(crate) fn save_for_after(globals: &mut TeXGlobals, t: cur_tok_type) {
    // begin if cur_level>level_one then
    if globals.cur_level > level_one {
        // begin check_full_save_stack;
        check_full_save_stack!(globals);
        // save_type(save_ptr):=insert_token; save_level(save_ptr):=level_zero;
        save_type!(globals, globals.save_ptr) = insert_token;
        save_level!(globals, globals.save_ptr) = level_zero;
        // save_index(save_ptr):=t; incr(save_ptr);
        #[cfg(not(feature = "unicode_support"))]
        {
            save_index!(globals, globals.save_ptr) = t.get() as _;
        }
        #[cfg(feature = "unicode_support")]
        {
            save_index!(globals, globals.save_ptr) =
                crate::unicode_support::register_info_value(globals, t.get());
        }
        incr!(globals.save_ptr);
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0221::level_one;
use crate::section_0221::level_zero;
use crate::section_0268::insert_token;
use crate::section_0268::save_index;
use crate::section_0268::save_level;
use crate::section_0268::save_type;
use crate::section_0273::check_full_save_stack;
use crate::section_0297::cur_tok_type;
