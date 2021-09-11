//! @ We've already defined the special variable |loc==cur_input.loc_field|
//! in our discussion of basic input-output routines. The other components of
//! |cur_input| are defined in the same way:

// @d state==cur_input.state_field {current scanner state}
/// current scanner state
pub(crate) macro state($globals:expr) {
    $globals.cur_input.state_field
}

// @d index==cur_input.index_field {reference for buffer information}
/// reference for buffer information
pub(crate) macro index($globals:expr) {
    $globals.cur_input.index_field
}

// @d start==cur_input.start_field {starting position in |buffer|}
/// starting position in `buffer`
pub(crate) macro start($globals:expr) {
    $globals.cur_input.start_field
}

// @d limit==cur_input.limit_field {end of current line in |buffer|}
/// end of current line in `buffer`
pub(crate) macro limit($globals:expr) {
    $globals.cur_input.limit_field
}

// @d name==cur_input.name_field {name of the current file}
/// name of the current file
pub(crate) macro name($globals:expr) {
    $globals.cur_input.name_field
}

crate::migration_complete!();
