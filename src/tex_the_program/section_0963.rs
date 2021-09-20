//! @ When the following code comes into play, the pattern $p_1\ldots p_k$
//! appears in |hc[1..k]|, and the corresponding sequence of numbers $n_0\ldots
//! n_k$ appears in |hyf[0..k]|.
//
// @<Insert a new pattern into the linked trie@>=
pub(crate) macro Insert_a_new_pattern_into_the_linked_trie($globals:expr, $k:expr, $l:expr) {{
    /// nodes of trie traversed during insertion
    let (mut p, mut q): (trie_pointer, trie_pointer);

    /// trie op code
    let mut v: quarterword;
    // begin @<Compute the trie op code, |v|, and set |l:=0|@>;
    crate::section_0965::Compute_the_trie_op_code__v__and_set_l_to_0!($globals, $k, $l, v);
    // q:=0; hc[0]:=cur_lang;
    q = 0.into();
    $globals.hc[0] = $globals.cur_lang.numeric_value() as _;
    // while l<=k do
    while $l <= $k {
        /// is `p=trie_l[q]`?
        let mut first_child: boolean;
        /// character being inserted
        let c: ASCII_code;
        // begin c:=hc[l]; incr(l); p:=trie_l[q]; first_child:=true;
        c = ASCII_code::from($globals.hc[$l.get() as usize] as integer);
        incr!($l);
        p = $globals.trie_l[q];
        first_child = true;
        // while (p>0)and(c>so(trie_c[p])) do
        while p > 0 && c > trie_c!($globals, p) {
            // begin q:=p; p:=trie_r[q]; first_child:=false;
            q = p;
            p = $globals.trie_r[q];
            first_child = false;
            // end;
        }
        // if (p=0)or(c<so(trie_c[p])) then
        if p == 0 || c < trie_c!($globals, p) {
            // @<Insert a new trie node between |q| and |p|, and
            //   make |p| point to it@>;
            crate::section_0964::Insert_a_new_trie_node_between_q_and_p__and_make_p_point_to_it!(
                $globals,
                p,
                q,
                c,
                first_child
            );
        }
        // q:=p; {now node |q| represents $p_1\ldots p_{l-1}$}
        /// now node `q` represents `p_1 ... p_{l-1}`
        const _: () = ();
        q = p;
        // end;
    }
    // if trie_o[q]<>min_quarterword then
    if $globals.trie_o[q] != min_quarterword {
        // begin print_err("Duplicate pattern");
        print_err!($globals, crate::strpool_str!("Duplicate pattern"));
        // @.Duplicate pattern@>
        // help1("(See Appendix H.)"); error;
        help1!($globals, crate::strpool_str!("(See Appendix H.)"));
        error($globals)?;
        // end;
    }
    // trie_o[q]:=v;
    $globals.trie_o[q] = v;
    // end
    use crate::pascal::boolean;
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0073::print_err;
    use crate::section_0079::help1;
    use crate::section_0082::error;
    use crate::section_0110::min_quarterword;
    use crate::section_0113::quarterword;
    use crate::section_0920::trie_pointer;
    use crate::section_0947::trie_c;
}}
