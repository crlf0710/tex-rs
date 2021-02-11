//! @ After |new_trie_op| has compressed the necessary opcode information,
//! plenty of information is available to unscramble the data into the
//! final form needed by our hyphenation algorithm.
//
// @<Sort \(t)the hyphenation op tables into proper order@>=
macro_rules! Sort_the_hyphenation_op_tables_into_proper_order {
    ($globals:expr) => {
        #[allow(unused_variables)]
        {
            // op_start[0]:=-min_quarterword;
            $globals.op_start[0] = ((0 - min_quarterword) as u16).into();
            // for j:=1 to 255 do op_start[j]:=op_start[j-1]+qo(trie_used[j-1]);
            for j in 1..=255 {
                $globals.op_start[j] =
                    $globals.op_start[j - 1] + qo!($globals.trie_used[j - 1]) as u16;
            }
            // for j:=1 to trie_op_ptr do
            for j in 1..=$globals.trie_op_ptr.get() {
                // trie_op_hash[j]:=op_start[trie_op_lang[j]]+trie_op_val[j]; {destination}
                /// destination
                const _: () = ();
                $globals.trie_op_hash[j as i16] = $globals.op_start
                    [$globals.trie_op_lang[j].numeric_value() as usize]
                    + $globals.trie_op_val[j] as trie_pointer_repr;
            }
            // for j:=1 to trie_op_ptr do while trie_op_hash[j]>j do
            for j in 1..=$globals.trie_op_ptr.get() {
                while $globals.trie_op_hash[j as i16] > j {
                    todo!("sort 2");
                    // begin k:=trie_op_hash[j];@/
                    // t:=hyf_distance[k]; hyf_distance[k]:=hyf_distance[j]; hyf_distance[j]:=t;@/
                    // t:=hyf_num[k]; hyf_num[k]:=hyf_num[j]; hyf_num[j]:=t;@/
                    // t:=hyf_next[k]; hyf_next[k]:=hyf_next[j]; hyf_next[j]:=t;@/
                    // trie_op_hash[j]:=trie_op_hash[k]; trie_op_hash[k]:=k;
                    // end
                }
            }
            use crate::section_0110::min_quarterword;
            use crate::section_0920::trie_pointer_repr;
        }
    };
}
