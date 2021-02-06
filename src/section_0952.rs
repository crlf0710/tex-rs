//! @ Here is how the trie-compression data structures are initialized.
//! If storage is tight, it would be possible to overlap |trie_op_hash|,
//! |trie_op_lang|, and |trie_op_val| with |trie|, |trie_hash|, and |trie_taken|,
//! because we finish with the former just before we need the latter.
//!
//! @<Get ready to compress the trie@>=
//! @<Sort \(t)the hyphenation...@>;
//! for p:=0 to trie_size do trie_hash[p]:=0;
//! trie_root:=compress_trie(trie_root); {identify equivalent subtries}
//! for p:=0 to trie_ptr do trie_ref[p]:=0;
//! for p:=0 to 255 do trie_min[p]:=p+1;
//! trie_link(0):=1; trie_max:=0
//!
