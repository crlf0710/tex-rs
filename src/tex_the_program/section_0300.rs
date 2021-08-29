//! @* \[22] Input stacks and states.
//! This implementation of
//! \TeX\ uses two different conventions for representing sequential stacks.
//! @^stack conventions@>@^conventions for representing stacks@>
//!
//! \yskip\hangg 1) If there is frequent access to the top entry, and if the
//! stack is essentially never empty, then the top entry is kept in a global
//! variable (even better would be a machine register), and the other entries
//! appear in the array $\\{stack}[0\to(\\{ptr}-1)]$. For example, the
//! semantic stack described above is handled this way, and so is the input
//! stack that we are about to study.
//!
//! \yskip\hangg 2) If there is infrequent top access, the entire stack contents
//! are in the array $\\{stack}[0\to(\\{ptr}-1)]$. For example, the |save_stack|
//! is treated this way, as we have seen.
//!
//! \yskip\noindent
//! The state of \TeX's input mechanism appears in the input stack, whose
//! entries are records with six fields, called |state|, |index|, |start|, |loc|,
//! |limit|, and |name|. This stack is maintained with
//! convention~(1), so it is declared in the following way:

// @<Types...@>=

// @!in_state_record = record
//   @!state_field, @!index_field: quarterword;
//   @!start_field,@!loc_field, @!limit_field, @!name_field: halfword;
//   end;
#[derive(Default, Clone, Copy)]
pub(crate) struct in_state_record {
    pub(crate) state_field: quarterword,
    pub(crate) index_field: quarterword,
    pub(crate) start_field: halfword,
    pub(crate) loc_field: halfword,
    pub(crate) limit_field: halfword,
    pub(crate) name_field: halfword,
}

use crate::section_0113::halfword;
use crate::section_0113::quarterword;
