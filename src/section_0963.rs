//! @ When the following code comes into play, the pattern $p_1\ldots p_k$
//! appears in |hc[1..k]|, and the corresponding sequence of numbers $n_0\ldots
//! n_k$ appears in |hyf[0..k]|.
//
// @<Insert a new pattern into the linked trie@>=
macro_rules! Insert_a_new_pattern_into_the_linked_trie {
    ($globals:expr, $k:expr, $l:expr) => {{
        /// nodes of trie traversed during insertion
        let (_p, _q): (trie_pointer, trie_pointer);

        /// trie op code
        let mut v: quarterword;
        // begin @<Compute the trie op code, |v|, and set |l:=0|@>;
        Compute_the_trie_op_code__v__and_set_l_to_0!($globals, $k, $l, v);
        // q:=0; hc[0]:=cur_lang;
        // while l<=k do
        //   begin c:=hc[l]; incr(l); p:=trie_l[q]; first_child:=true;
        //   while (p>0)and(c>so(trie_c[p])) do
        //     begin q:=p; p:=trie_r[q]; first_child:=false;
        //     end;
        //   if (p=0)or(c<so(trie_c[p])) then
        //     @<Insert a new trie node between |q| and |p|, and
        //       make |p| point to it@>;
        //   q:=p; {now node |q| represents $p_1\ldots p_{l-1}$}
        //   end;
        // if trie_o[q]<>min_quarterword then
        //   begin print_err("Duplicate pattern");
        // @.Duplicate pattern@>
        //   help1("(See Appendix H.)"); error;
        //   end;
        // trie_o[q]:=v;
        // end
        todo!("insert a new pattern");
        use crate::section_0110::min_quarterword;
        use crate::section_0113::quarterword;
        use crate::section_0920::trie_pointer;
    }}
}
