//! @ Each time \.{\\patterns} appears, it contributes further patterns to
//! the future trie, which will be built only when hyphenation is attempted or
//! when a format file is dumped. The boolean variable |trie_not_ready|
//! will change to |false| when the trie is compressed; this will disable
//! further patterns.
//!
//! @<Initialize table entries...@>=
//! trie_not_ready:=true; trie_root:=0; trie_c[0]:=si(0); trie_ptr:=0;
//!
