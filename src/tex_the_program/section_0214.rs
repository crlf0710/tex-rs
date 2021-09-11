//! @ Here is a common way to make the current list grow:
//
// @d tail_append(#)==begin link(tail):=#; tail:=link(tail);
pub(crate) macro tail_append($globals:expr, $ptr:expr) {
    link!($globals, tail!($globals)) = $ptr;
    tail!($globals) = link!($globals, tail!($globals));
    // end
    use crate::section_0118::link;
    use crate::section_0213::tail;
}
