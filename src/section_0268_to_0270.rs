//! @* \[19] Saving and restoring equivalents.
//! The nested structure provided by `$\.{\char'173}\ldots\.{\char'175}$' groups
//! in \TeX\ means that |eqtb| entries valid in outer groups should be saved
//! and restored later if they are overridden inside the braces. When a new |eqtb|
//! value is being assigned, the program therefore checks to see if the previous
//! entry belongs to an outer level. In such a case, the old value is placed
//! on the |save_stack| just before the new value enters |eqtb|. At the
//! end of a grouping level, i.e., when the right brace is sensed, the
//! |save_stack| is used to restore the outer values, and the inner ones are
//! destroyed.
//!
//! Entries on the |save_stack| are of type |memory_word|. The top item on
//! this stack is |save_stack[p]|, where |p=save_ptr-1|; it contains three
//! fields called |save_type|, |save_level|, and |save_index|, and it is
//! interpreted in one of four ways:
//!
//! \yskip\hangg 1) If |save_type(p)=restore_old_value|, then
//! |save_index(p)| is a location in |eqtb| whose current value should
//! be destroyed at the end of the current group and replaced by |save_stack[p-1]|.
//! Furthermore if |save_index(p)>=int_base|, then |save_level(p)|
//! should replace the corresponding entry in |xeq_level|.
//!
//! \yskip\hangg 2) If |save_type(p)=restore_zero|, then |save_index(p)|
//! is a location in |eqtb| whose current value should be destroyed at the end
//! of the current group, when it should be
//! replaced by the current value of |eqtb[undefined_control_sequence]|.
//!
//! \yskip\hangg 3) If |save_type(p)=insert_token|, then |save_index(p)|
//! is a token that should be inserted into \TeX's input when the current
//! group ends.
//!
//! \yskip\hangg 4) If |save_type(p)=level_boundary|, then |save_level(p)|
//! is a code explaining what kind of group we were previously in, and
//! |save_index(p)| points to the level boundary word at the bottom of
//! the entries for that group.
//!
//! @d save_type(#)==save_stack[#].hh.b0 {classifies a |save_stack| entry}
//! @d save_level(#)==save_stack[#].hh.b1
//!   {saved level for regions 5 and 6, or group code}
//! @d save_index(#)==save_stack[#].hh.rh
//!   {|eqtb| location or token or |save_stack| location}
//! @d restore_old_value=0 {|save_type| when a value should be restored later}
//! @d restore_zero=1 {|save_type| when an undefined entry should be restored}
//! @d insert_token=2 {|save_type| when a token is being saved for later use}
//! @d level_boundary=3 {|save_type| corresponding to beginning of group}
//!
//! @ Here are the group codes that are used to discriminate between different
//! kinds of groups. They allow \TeX\ to decide what special actions, if any,
//! should be performed when a group ends.
//! \def\grp{\.{\char'173...\char'175}}
//!
//! Some groups are not supposed to be ended by right braces. For example,
//! the `\.\$' that begins a math formula causes a |math_shift_group| to
//! be started, and this should be terminated by a matching `\.\$'. Similarly,
//! a group that starts with \.{\\left} should end with \.{\\right}, and
//! one that starts with \.{\\begingroup} should end with \.{\\endgroup}.
//!
//! @d bottom_level=0 {group code for the outside world}
//! @d simple_group=1 {group code for local structure only}
//! @d hbox_group=2 {code for `\.{\\hbox}\grp'}
//! @d adjusted_hbox_group=3 {code for `\.{\\hbox}\grp' in vertical mode}
//! @d vbox_group=4 {code for `\.{\\vbox}\grp'}
//! @d vtop_group=5 {code for `\.{\\vtop}\grp'}
//! @d align_group=6 {code for `\.{\\halign}\grp', `\.{\\valign}\grp'}
//! @d no_align_group=7 {code for `\.{\\noalign}\grp'}
//! @d output_group=8 {code for output routine}
//! @d math_group=9 {code for, e.g., `\.{\char'136}\grp'}
//! @d disc_group=10 {code for `\.{\\discretionary}\grp\grp\grp'}
//! @d insert_group=11 {code for `\.{\\insert}\grp', `\.{\\vadjust}\grp'}
//! @d vcenter_group=12 {code for `\.{\\vcenter}\grp'}
//! @d math_choice_group=13 {code for `\.{\\mathchoice}\grp\grp\grp\grp'}
//! @d semi_simple_group=14 {code for `\.{\\begingroup...\\endgroup}'}
//! @d math_shift_group=15 {code for `\.{\$...\$}'}
//! @d math_left_group=16 {code for `\.{\\left...\\right}'}
//! @d max_group_code=16
//!
//! @<Types...@>=
//! @!group_code=0..max_group_code; {|save_level| for a level boundary}
//!
//! @ The global variable |cur_group| keeps track of what sort of group we are
//! currently in. Another global variable, |cur_boundary|, points to the
//! topmost |level_boundary| word.  And |cur_level| is the current depth of
//! nesting. The routines are designed to preserve the condition that no entry
//! in the |save_stack| or in |eqtb| ever has a level greater than |cur_level|.
//!
