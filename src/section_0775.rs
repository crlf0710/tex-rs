//! @ In vertical modes, |prev_depth| already has the correct value. But
//! if we are in |mmode| (displayed formula mode), we reach out to the
//! enclosing vertical mode for the |prev_depth| value that produces the
//! correct baseline calculations.
//
// @<Change current mode...@>=
macro_rules! Change_current_mode_to_neg_vmode_for_halign__neg_hmode_for_valign {
    ($globals:expr) => {{
        // if mode=mmode then
        if mode!($globals) == mmode {
            // begin mode:=-vmode; prev_depth:=nest[nest_ptr-2].aux_field.sc;
            mode!($globals) = (-vmode).into();
            prev_depth!($globals) = $globals.nest[$globals.nest_ptr - 2].aux_field[MEMORY_WORD_SC];
            // end
        }
        // else if mode>0 then negate(mode)
        else {
            if mode!($globals) > 0 {
                negate!(mode!($globals));
            }
        }
        use crate::section_0101::MEMORY_WORD_SC;
        use crate::section_0211::vmode;
        use crate::section_0211::mmode;
    }}
}

