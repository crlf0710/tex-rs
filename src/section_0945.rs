//! @ After |new_trie_op| has compressed the necessary opcode information,
//! plenty of information is available to unscramble the data into the
//! final form needed by our hyphenation algorithm.
//!
//! @<Sort \(t)the hyphenation op tables into proper order@>=
//! op_start[0]:=-min_quarterword;
//! for j:=1 to 255 do op_start[j]:=op_start[j-1]+qo(trie_used[j-1]);
//! for j:=1 to trie_op_ptr do
//!   trie_op_hash[j]:=op_start[trie_op_lang[j]]+trie_op_val[j]; {destination}
//! for j:=1 to trie_op_ptr do while trie_op_hash[j]>j do
//!   begin k:=trie_op_hash[j];@/
//!   t:=hyf_distance[k]; hyf_distance[k]:=hyf_distance[j]; hyf_distance[j]:=t;@/
//!   t:=hyf_num[k]; hyf_num[k]:=hyf_num[j]; hyf_num[j]:=t;@/
//!   t:=hyf_next[k]; hyf_next[k]:=hyf_next[j]; hyf_next[j]:=t;@/
//!   trie_op_hash[j]:=trie_op_hash[k]; trie_op_hash[k]:=k;
//!   end
//!
