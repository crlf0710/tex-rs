//! @ At this time it might be a good idea for the reader to review the introduction
//! to |eqtb| that was given above just before the long lists of parameter names.
//! Recall that the ``outer level'' of the program is |level_one|, since
//! undefined control sequences are assumed to be ``defined'' at |level_zero|.

// @<Set init...@>=
pub(crate) macro Set_initial_values_of_key_variables_0272($globals:expr) {
    let globals = &mut *$globals;
    // save_ptr:=0; cur_level:=level_one; cur_group:=bottom_level; cur_boundary:=0;
    globals.save_ptr = 0.into();
    globals.cur_level = level_one;
    globals.cur_group = bottom_level.into();
    globals.cur_boundary = 0.into();
    // max_save_stack:=0;
    globals.max_save_stack = 0.into();

    use crate::section_0073::error_stop_mode;
    use crate::section_0221::level_one;
    use crate::section_0269::bottom_level;
}
