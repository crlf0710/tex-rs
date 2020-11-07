//! @ Procedure |new_save_level| is called when a group begins. The
//! argument is a group identification code like `|hbox_group|'. After
//! calling this routine, it is safe to put five more entries on |save_stack|.
//!
//! In some cases integer-valued items are placed onto the
//! |save_stack| just below a |level_boundary| word, because this is a
//! convenient place to keep information that is supposed to ``pop up'' just
//! when the group has finished.
//! For example, when `\.{\\hbox to 100pt}\grp' is being treated, the 100pt
//! dimension is stored on |save_stack| just before |new_save_level| is
//! called.
//!
//! We use the notation |saved(k)| to stand for an integer item that
//! appears in location |save_ptr+k| of the save stack.
//!
//! @d saved(#)==save_stack[save_ptr+#].int
//!
//! @p procedure new_save_level(@!c:group_code); {begin a new level of grouping}
//! begin check_full_save_stack;
//! save_type(save_ptr):=level_boundary; save_level(save_ptr):=cur_group;
//! save_index(save_ptr):=cur_boundary;
//! if cur_level=max_quarterword then overflow("grouping levels",
//! @:TeX capacity exceeded grouping levels}{\quad grouping levels@>
//!   max_quarterword-min_quarterword);
//!   {quit if |(cur_level+1)| is too big to be stored in |eqtb|}
//! cur_boundary:=save_ptr; incr(cur_level); incr(save_ptr); cur_group:=c;
//! end;
//!
