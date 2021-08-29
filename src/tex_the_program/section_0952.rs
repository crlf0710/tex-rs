//! @ Here is how the trie-compression data structures are initialized.
//! If storage is tight, it would be possible to overlap |trie_op_hash|,
//! |trie_op_lang|, and |trie_op_val| with |trie|, |trie_hash|, and |trie_taken|,
//! because we finish with the former just before we need the latter.
//
// @<Get ready to compress the trie@>=
macro_rules! Get_ready_to_compress_the_trie {
    ($globals:expr) => {{
        // @<Sort \(t)the hyphenation...@>;
        Sort_the_hyphenation_op_tables_into_proper_order!($globals);
        // for p:=0 to trie_size do trie_hash[p]:=0;
        for p in 0..=trie_size {
            $globals.trie_hash[p] = 0.into();
        }
        // trie_root:=compress_trie(trie_root); {identify equivalent subtries}
        /// identify equivalent subtries
        const _ : () = ();
        trie_root!($globals) = compress_trie($globals, trie_root!($globals));
        // for p:=0 to trie_ptr do trie_ref[p]:=0;
        for p in 0..=$globals.trie_ptr.get() {
            trie_ref!($globals, p) = 0.into();
        }
        // for p:=0 to 255 do trie_min[p]:=p+1;
        for p in 0..=255 {
            $globals.trie_min[p as usize] = (p + 1).into();
        }
        // trie_link(0):=1; trie_max:=0
        trie_link!($globals, 0) = 1;
        $globals.trie_max = 0.into();

        use crate::section_0011::trie_size;
        use crate::section_0949::compress_trie;
    }}
}

