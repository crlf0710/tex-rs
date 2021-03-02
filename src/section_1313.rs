//! ` `
// @<Dump the table of equivalents@>=
macro_rules! Dump_the_table_of_equivalents {
    ($globals:expr) => {{
        let k: integer;
        // @<Dump regions 1 to 4 of |eqtb|@>;
        Dump_regions_1_to_4_of_eqtb!($globals, k);
        // @<Dump regions 5 and 6 of |eqtb|@>;
        Dump_regions_5_and_6_of_eqtb!($globals, k);
        // dump_int(par_loc); dump_int(write_loc);@/
        dump_int!($globals, $globals.par_loc as _);
        dump_int!($globals, $globals.write_loc as _);
        // @<Dump the hash table@>
        Dump_the_hash_table!($globals);
        use crate::pascal::integer;
    }}
}
