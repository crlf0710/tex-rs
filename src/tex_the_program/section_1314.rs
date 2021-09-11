//! ` `
// @<Undump the table of equivalents@>=
pub(crate) macro Undump_the_table_of_equivalents($globals:expr, $lbl_bad_fmt:lifetime) {{
    // @<Undump regions 1 to 6 of |eqtb|@>;
    crate::section_1317::Undump_regions_1_to_6_of_eqtb!($globals, $lbl_bad_fmt);
    // undump(hash_base)(frozen_control_sequence)(par_loc);
    undump!(
        $globals,
        hash_base,
        frozen_control_sequence,
        $globals.par_loc,
        core::convert::identity,
        $lbl_bad_fmt
    );
    // par_token:=cs_token_flag+par_loc;@/
    $globals.par_token = cur_tok_type::from_cs($globals.par_loc);
    // undump(hash_base)(frozen_control_sequence)(write_loc);@/
    undump!(
        $globals,
        hash_base,
        frozen_control_sequence,
        $globals.write_loc,
        core::convert::identity,
        $lbl_bad_fmt
    );
    // @<Undump the hash table@>
    crate::section_1319::Undump_the_hash_table!($globals, $lbl_bad_fmt);
    use crate::section_0222::frozen_control_sequence;
    use crate::section_0222::hash_base;
    use crate::section_0297::cur_tok_type;
    use crate::section_1306::undump;
}}
