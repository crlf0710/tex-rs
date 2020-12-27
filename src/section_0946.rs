//! @ Before we forget how to initialize the data structures that have been
//! mentioned so far, let's write down the code that gets them started.
//!
//! @<Initialize table entries...@>=
//! for k:=-trie_op_size to trie_op_size do trie_op_hash[k]:=0;
//! for k:=0 to 255 do trie_used[k]:=min_quarterword;
//! trie_op_ptr:=0;
//!
