//! @ A different scheme is used to compress the hash table, since its lower
//! region is usually sparse. When |text(p)<>0| for |p<=hash_used|, we output
//! two words, |p| and |hash[p]|. The hash table is, of course, densely packed
//! for |p>=hash_used|, so the remaining entries are output in a~block.
//
// @<Dump the hash table@>=
macro_rules! Dump_the_hash_table {
    ($globals:expr) => {{
        // dump_int(hash_used); cs_count:=frozen_control_sequence-1-hash_used;
        dump_int!($globals, $globals.hash_used as _);
        $globals.cs_count = frozen_control_sequence as integer - 1 - $globals.hash_used as integer;
        // for p:=hash_base to hash_used do if text(p)<>0 then
        for p in hash_base as _ ..= $globals.hash_used {
            if text!($globals, p) != 0 {
                // begin dump_int(p); dump_hh(hash[p]); incr(cs_count);
                dump_int!($globals, p as _);
                dump_hh!($globals, $globals.hash[p]);
                incr!($globals.cs_count);
                // end;
            }
        }
        // for p:=hash_used+1 to undefined_control_sequence-1 do dump_hh(hash[p]);
        for p in $globals.hash_used + 1 ..= undefined_control_sequence - 1 {
            dump_hh!($globals, $globals.hash[p]);
        }
        // dump_int(cs_count);@/
        dump_int!($globals, $globals.cs_count);
        // print_ln; print_int(cs_count); print(" multiletter control sequences")
        print_ln(make_globals_io_string_log_view!($globals));
        print_int($globals, $globals.cs_count);
        print($globals, strpool_str!(" multiletter control sequences").get() as _);
        use crate::pascal::integer;
        use crate::section_0004::TeXGlobalsIoStringLogView;
        use crate::section_0057::print_ln;
        use crate::section_0059::print;
        use crate::section_0065::print_int;
        use crate::section_0222::undefined_control_sequence;
        use crate::section_0222::frozen_control_sequence;
        use crate::section_0222::hash_base;
    }}
}
