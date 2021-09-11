//! @ When the whole trie has been allocated into the sequential table, we
//! must go through it once again so that |trie| contains the correct
//! information. Null pointers in the linked trie will be represented by the
//! value~0, which properly implements an ``empty'' family.
//
// @<Move the data into |trie|@>=
pub(crate) macro Move_the_data_into_trie($globals:expr) {{
    crate::trace_span!("Move the data into `trie`");
    /// template used to zero out `trie`'s holes
    let mut h: two_halves = two_halves::default();
    // h.rh:=0; h.b0:=min_quarterword; h.b1:=min_quarterword; {|trie_link:=0|,
    //   |trie_op:=min_quarterword|, |trie_char:=qi(0)|}
    /// `trie_link:=0`, `trie_op:=min_quarterword`, `trie_char:=qi(0)`
    const _: () = ();
    h[TWO_HALVES_RH] = 0;
    #[cfg(not(feature = "unicode_support"))]
    {
        h[TWO_HALVES_LH_B0] = min_quarterword;
        h[TWO_HALVES_LH_B1] = min_quarterword;
    }
    #[cfg(feature = "unicode_support")]
    {
        h[TWO_HALVES_LH] = crate::unicode_support::register_triecharop_value(
            $globals,
            crate::section_0921::trie_char_and_op {
                char: ASCII_code::from(0),
                op: 0,
            },
        );
        use crate::section_0113::TWO_HALVES_LH;
    }
    // if trie_root=0 then {no patterns were given}
    if trie_root!($globals) == 0 {
        /// no patterns were given
        const _: () = ();
        // begin for r:=0 to 256 do trie[r]:=h;
        for r in 0..=256 {
            $globals.trie[r] = h;
        }
        // trie_max:=256;
        $globals.trie_max = 256.into();
        // end
    }
    // else begin trie_fix(trie_root); {this fixes the non-holes in |trie|}
    else {
        /// this fixes the non-holes in `trie`
        const _: () = ();

        /// used to clean up the packed `trie`
        let (mut r, mut s): (trie_pointer, trie_pointer);

        trie_fix($globals, trie_root!($globals));
        // r:=0; {now we will zero out all the holes}
        /// now we will zero out all the holes
        const _: () = ();
        r = 0.into();
        // repeat s:=trie_link(r); trie[r]:=h; r:=s;
        loop {
            s = trie_link!($globals, r.get()).into();
            $globals.trie[r.get()] = h;
            r = s;
            // until r>trie_max;
            if r > $globals.trie_max {
                break;
            }
        }
        // end;
    }
    // trie_char(0):=qi("?"); {make |trie_char(c)<>c| for all |c|}
    /// make `trie_char(c)<>c` for all `c`
    const _: () = ();
    let trie_op = trie_op!($globals, 0);
    assign_trie_char_and_op!($globals, 0, ASCII_code_literal!(b'?'), trie_op);
    use crate::section_0018::ASCII_code;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0113::two_halves;
    use crate::section_0113::TWO_HALVES_RH;
    use crate::section_0920::trie_pointer;
    use crate::section_0921::assign_trie_char_and_op;
    use crate::section_0921::trie_link;
    use crate::section_0921::trie_op;
    use crate::section_0947::trie_root;
    use crate::section_0959::trie_fix;
}}
