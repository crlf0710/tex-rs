//! ` `
// @<Make a copy of node |p|...@>=
pub(crate) macro Make_a_copy_of_node_p_in_node_r($globals:expr, $p:expr, $r:expr) {{
    /// number of words remaining to be copied
    let mut words;
    // words:=1; {this setting occurs in more branches than any other}
    words = 1;
    /// this setting occurs in more branches than any other
    const _: () = ();
    // if is_char_node(p) then r:=get_avail
    if is_char_node!($globals, $p) {
        $r = get_avail($globals);
    }
    // else @<Case statement to copy different types and set |words| to the number
    //   of initial words not yet copied@>;
    else {
        crate::section_0206::Case_statement_to_copy_different_types_and_set_words_to_the_number_of_initial_words_not_yet_copied!($globals, $p, $r, words);
    }
    // while words>0 do
    while words > 0 {
        // begin decr(words); mem[r+words]:=mem[p+words];
        decr!(words);
        $globals.mem[$r + words] = $globals.mem[$p + words];
        // end
    }
    use crate::section_0016::decr;
    use crate::section_0120::get_avail;
    use crate::section_0134::is_char_node;
}}
