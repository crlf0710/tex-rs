//! ` `
// @<Dump the hyphenation tables@>=
pub(crate) macro Dump_the_hyphenation_tables($globals:expr) {{
    // dump_int(hyph_count);
    dump_int!($globals, $globals.hyph_count.get() as _);
    // for k:=0 to hyph_size do if hyph_word[k]<>0 then
    for k in 0..=hyph_size as _ {
        if $globals.hyph_word[k] != 0 {
            // begin dump_int(k); dump_int(hyph_word[k]); dump_int(hyph_list[k]);
            dump_int!($globals, k as _);
            dump_int!($globals, $globals.hyph_word[k].get() as _);
            dump_int!($globals, $globals.hyph_list[k] as _);
            // end;
        }
    }
    // print_ln; print_int(hyph_count); print(" hyphenation exception");
    print_ln(make_globals_io_string_log_view!($globals));
    print_int($globals, $globals.hyph_count.get() as _);
    print(
        $globals,
        crate::strpool_str!(" hyphenation exception").get() as _,
    );
    // if hyph_count<>1 then print_char("s");
    if $globals.hyph_count != 1 {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b's'),
        );
    }
    // if trie_not_ready then init_trie;
    if $globals.trie_not_ready {
        init_trie($globals)?;
    }
    // dump_int(trie_max);
    dump_int!($globals, $globals.trie_max.get() as _);
    // for k:=0 to trie_max do dump_hh(trie[k]);
    for k in 0..=$globals.trie_max.get() {
        dump_hh!($globals, $globals.trie[k]);
    }
    // dump_int(trie_op_ptr);
    dump_int!($globals, $globals.trie_op_ptr.get() as _);
    // for k:=1 to trie_op_ptr do
    for k in 1..=$globals.trie_op_ptr.get() {
        // begin dump_int(hyf_distance[k]);
        dump_int!($globals, $globals.hyf_distance[k].get() as _);
        // dump_int(hyf_num[k]);
        dump_int!($globals, $globals.hyf_num[k].get() as _);
        // dump_int(hyf_next[k]);
        dump_int!($globals, $globals.hyf_next[k] as _);
        // end;
    }
    // print_nl("Hyphenation trie of length "); print_int(trie_max);
    print_nl($globals, crate::strpool_str!("Hyphenation trie of length "));
    print_int($globals, $globals.trie_max.get() as _);
    // @.Hyphenation trie...@>
    // print(" has "); print_int(trie_op_ptr); print(" op");
    print($globals, crate::strpool_str!(" has ").get() as _);
    print_int($globals, $globals.trie_op_ptr.get() as _);
    print($globals, crate::strpool_str!(" op").get() as _);
    // if trie_op_ptr<>1 then print_char("s");
    if $globals.trie_op_ptr.get() != 1 {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b's'),
        );
    }
    // print(" out of "); print_int(trie_op_size);
    print($globals, crate::strpool_str!(" out of ").get() as _);
    print_int($globals, trie_op_size as _);
    // for k:=255 downto 0 do if trie_used[k]>min_quarterword then
    for k in (0..=255).rev() {
        if $globals.trie_used[k] > min_quarterword {
            // begin print_nl("  "); print_int(qo(trie_used[k]));
            print_nl($globals, crate::strpool_str!("  "));
            print_int($globals, $globals.trie_used[k] as _);
            // print(" for language "); print_int(k);
            print($globals, crate::strpool_str!(" for language ").get() as _);
            print_int($globals, k as _);
            // dump_int(k); dump_int(qo(trie_used[k]));
            dump_int!($globals, k as _);
            dump_int!($globals, $globals.trie_used[k] as _);
            // end
        }
    }
    use crate::section_0004::make_globals_io_string_log_view;
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0011::trie_op_size;
    use crate::section_0012::hyph_size;
    use crate::section_0018::ASCII_code_literal;
    use crate::section_0057::print_ln;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0065::print_int;
    use crate::section_0110::min_quarterword;
    use crate::section_0966::init_trie;
    use crate::section_1305::dump_hh;
    use crate::section_1305::dump_int;
}}
