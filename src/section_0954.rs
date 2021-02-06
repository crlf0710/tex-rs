//! @ By making sure that |trie_max| is at least |h+256|, we can be sure that
//! |trie_max>z|, since |h=z-c|. It follows that location |trie_max| will
//! never be occupied in |trie|, and we will have |trie_max>=trie_link(z)|.
//
// @<Ensure that |trie_max>=h+256|@>=
macro_rules! Ensure_that_trie_max_ge_h_plus_256 {
    ($globals:expr, $h:expr) => {{
        // if trie_max<h+256 then
        if $globals.trie_max < $h + 256 {
            // begin if trie_size<=h+256 then overflow("pattern memory",trie_size);
            if trie_size <= $h.get() + 256 {
                overflow($globals, strpool_str!("pattern memory"), trie_size as _)?;
            }
            // @:TeX capacity exceeded pattern memory}{\quad pattern memory@>
            // repeat incr(trie_max); trie_taken[trie_max]:=false;
            loop {
                incr!($globals.trie_max);
                $globals.trie_taken[$globals.trie_max.get()] = false;
                // trie_link(trie_max):=trie_max+1; trie_back(trie_max):=trie_max-1;
                trie_link!($globals, $globals.trie_max) = $globals.trie_max.get() + 1;
                trie_back!($globals, $globals.trie_max) = $globals.trie_max.get() - 1;
                // until trie_max=h+256;
                if $globals.trie_max == $h + 256 {
                    break;
                }
            }
            // end
        }
        use crate::section_0011::trie_size;
        use crate::section_0094::overflow;
    }}
}
