//! ` `

// @<Initialize table entries...@>=
#[allow(unused_variables)]
pub(crate) macro Initialize_table_entries_done_by_initex_only_0258($globals:expr) {{
    let globals = &mut *$globals;
    // hash_used:=frozen_control_sequence; {nothing is used}
    /// nothing is used
    const _: () = ();
    globals.hash_used = frozen_control_sequence as _;
    // cs_count:=0;
    globals.cs_count = 0;
    // eq_type(frozen_dont_expand):=dont_expand;
    eq_type!(globals, frozen_dont_expand) = dont_expand;
    // text(frozen_dont_expand):="notexpanded:";
    text!(globals, frozen_dont_expand as pointer) = crate::strpool_str!("notexpanded:").get() as _;
    // @.notexpanded:@>
}}

use crate::section_0004::TeXGlobals;
use crate::section_0115::pointer;
use crate::section_0210::dont_expand;
use crate::section_0221::eq_type;
use crate::section_0222::frozen_control_sequence;
use crate::section_0222::frozen_dont_expand;
use crate::section_0256::text;
