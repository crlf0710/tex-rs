//! ` `

// @<Insert a new trie node between |q| and |p|...@>=
pub(crate) macro Insert_a_new_trie_node_between_q_and_p__and_make_p_point_to_it($globals:expr, $p:expr, $q:expr, $c:expr, $first_child:expr) {{
    // begin if trie_ptr=trie_size then overflow("pattern memory",trie_size);
    if $globals.trie_ptr == trie_size {
        todo!("overflow");
    }
    // @:TeX capacity exceeded pattern memory}{\quad pattern memory@>
    // incr(trie_ptr); trie_r[trie_ptr]:=p; p:=trie_ptr; trie_l[p]:=0;
    incr!($globals.trie_ptr);
    $globals.trie_r[$globals.trie_ptr] = $p;
    $p = $globals.trie_ptr;
    $globals.trie_l[$p] = 0.into();
    // if first_child then trie_l[q]:=p@+else trie_r[q]:=p;
    if $first_child {
        $globals.trie_l[$q] = $p;
    } else {
        $globals.trie_r[$q] = $p;
    }
    // trie_c[p]:=si(c); trie_o[p]:=min_quarterword;
    trie_c_assign!($globals, $p, $c);
    $globals.trie_o[$p] = min_quarterword;
    // end
    use crate::section_0011::trie_size;
    use crate::section_0016::incr;
    use crate::section_0110::min_quarterword;
    use crate::section_0947::trie_c_assign;
}}
