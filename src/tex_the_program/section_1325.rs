//! @ Only ``nonempty'' parts of |op_start| need to be restored.
//
// @<Undump the hyphenation tables@>=
pub(crate) macro Undump_the_hyphenation_tables($globals:expr, $lbl_bad_fmt:lifetime) {{
    // undump(0)(hyph_size)(hyph_count);
    undump!(
        $globals,
        0,
        hyph_size,
        $globals.hyph_count,
        hyph_pointer::new,
        $lbl_bad_fmt
    );
    // for k:=1 to hyph_count do
    for k in 1..=$globals.hyph_count.get() {
        let j: hyph_pointer;
        // begin undump(0)(hyph_size)(j);
        undump!($globals, 0, hyph_size, j, hyph_pointer::new, $lbl_bad_fmt);
        // undump(0)(str_ptr)(hyph_word[j]);
        undump!(
            $globals,
            0,
            $globals.str_ptr.get(),
            $globals.hyph_word[j],
            str_number::new,
            $lbl_bad_fmt
        );
        // undump(min_halfword)(max_halfword)(hyph_list[j]);
        undump!(
            $globals,
            min_halfword,
            max_halfword,
            $globals.hyph_list[j],
            core::convert::identity,
            $lbl_bad_fmt
        );
        // end;
    }
    let j: trie_pointer;
    // undump_size(0)(trie_size)('trie size')(j); @+init trie_max:=j;@+tini
    undump_size!(
        $globals,
        0,
        trie_size,
        "trie size",
        j,
        trie_pointer::new,
        $lbl_bad_fmt
    );
    crate::region_initex! {
        $globals.trie_max = j;
    }
    // for k:=0 to j do undump_hh(trie[k]);
    for k in 0..=j.get() {
        undump_hh!($globals, $globals.trie[k]);
    }
    let mut j;
    // undump_size(0)(trie_op_size)('trie op size')(j); @+init trie_op_ptr:=j;@+tini
    undump_size!(
        $globals,
        0,
        trie_op_size,
        "trie op size",
        j,
        u16_from_0_to_n::new,
        $lbl_bad_fmt
    );
    crate::region_initex! {
        $globals.trie_op_ptr = j;
    }
    // for k:=1 to j do
    for k in 1..=j.get() {
        // begin undump(0)(63)(hyf_distance[k]); {a |small_number|}
        /// a `small_number`
        const _: () = ();
        undump!(
            $globals,
            0,
            63,
            $globals.hyf_distance[k],
            small_number::new,
            $lbl_bad_fmt
        );
        // undump(0)(63)(hyf_num[k]);
        undump!(
            $globals,
            0,
            63,
            $globals.hyf_num[k],
            small_number::new,
            $lbl_bad_fmt
        );
        // undump(min_quarterword)(max_quarterword)(hyf_next[k]);
        undump!(
            $globals,
            min_quarterword,
            max_quarterword,
            $globals.hyf_next[k],
            core::convert::identity,
            $lbl_bad_fmt
        );
        // end;
    }
    // init for k:=0 to 255 do trie_used[k]:=min_quarterword;@+tini@;@/
    crate::region_initex! {
        for k in 0..=255 {
            $globals.trie_used[k] = min_quarterword;
        }
    }
    // k:=256;
    let mut k = 256;
    // while j>0 do
    while j > 0 {
        let x: u16;
        // begin undump(0)(k-1)(k); undump(1)(j)(x);@+init trie_used[k]:=qi(x);@+tini@;@/
        undump!($globals, 0, k - 1, k, core::convert::identity, $lbl_bad_fmt);
        undump!(
            $globals,
            1,
            j.get(),
            x,
            core::convert::identity,
            $lbl_bad_fmt
        );
        crate::region_initex! {
            $globals.trie_used[k] = x as _;
        }
        // j:=j-x; op_start[k]:=qo(j);
        j = j.get().saturating_sub(x).into();
        $globals.op_start[k] = j;
        // end;
    }
    // @!init trie_not_ready:=false @+tini
    crate::region_initex! {
        $globals.trie_not_ready = false;
    }
    use crate::pascal::u16_from_0_to_n;
    use crate::section_0011::trie_op_size;
    use crate::section_0011::trie_size;
    use crate::section_0012::hyph_size;
    use crate::section_0038::str_number;
    use crate::section_0101::small_number;
    use crate::section_0110::max_halfword;
    use crate::section_0110::max_quarterword;
    use crate::section_0110::min_halfword;
    use crate::section_0110::min_quarterword;
    use crate::section_0920::trie_pointer;
    use crate::section_0925::hyph_pointer;
    use crate::section_1306::undump;
    use crate::section_1306::undump_hh;
    use crate::section_1306::undump_size;
}}
