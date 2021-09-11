//! ` `
// @<Dump the table of equivalents@>=
pub(crate) macro Dump_the_table_of_equivalents($globals:expr) {{
    let k: integer;
    // @<Dump regions 1 to 4 of |eqtb|@>;
    crate::section_1315::Dump_regions_1_to_4_of_eqtb!($globals, k);
    // @<Dump regions 5 and 6 of |eqtb|@>;
    crate::section_1316::Dump_regions_5_and_6_of_eqtb!($globals, k);
    // dump_int(par_loc); dump_int(write_loc);@/
    dump_int!($globals, $globals.par_loc as _);
    dump_int!($globals, $globals.write_loc as _);
    // @<Dump the hash table@>
    crate::section_1318::Dump_the_hash_table!($globals);
    use crate::pascal::integer;
    use crate::section_1305::dump_int;
}}
