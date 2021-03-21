//! @ Only ``nonempty'' parts of |op_start| need to be restored.
//
// @<Undump the hyphenation tables@>=
macro_rules! Undump_the_hyphenation_tables {
    ($globals:expr, $lbl_bad_fmt:lifetime) => {{
        // undump(0)(hyph_size)(hyph_count);
        // for k:=1 to hyph_count do
        //   begin undump(0)(hyph_size)(j);
        //   undump(0)(str_ptr)(hyph_word[j]);
        //   undump(min_halfword)(max_halfword)(hyph_list[j]);
        //   end;
        // undump_size(0)(trie_size)('trie size')(j); @+init trie_max:=j;@+tini
        // for k:=0 to j do undump_hh(trie[k]);
        // undump_size(0)(trie_op_size)('trie op size')(j); @+init trie_op_ptr:=j;@+tini
        // for k:=1 to j do
        //   begin undump(0)(63)(hyf_distance[k]); {a |small_number|}
        //   undump(0)(63)(hyf_num[k]);
        //   undump(min_quarterword)(max_quarterword)(hyf_next[k]);
        //   end;
        // init for k:=0 to 255 do trie_used[k]:=min_quarterword;@+tini@;@/
        // k:=256;
        // while j>0 do
        //   begin undump(0)(k-1)(k); undump(1)(j)(x);@+init trie_used[k]:=qi(x);@+tini@;@/
        //   j:=j-x; op_start[k]:=qo(j);
        //   end;
        // @!init trie_not_ready:=false @+tini
        panic!("Undump hyphenation");
    }}
}
