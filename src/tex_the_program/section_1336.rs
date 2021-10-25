//! ` `

// @ @<Last-minute...@>=

pub(crate) macro Put_each_of_TeX_s_primitivies_into_the_hash_table($globals:expr) {
    crate::trace_span!("Put each of TeX's primitivies into the hash table");
    crate::section_0226::Put_each_of_tex_s_primitivies_into_the_hash_table_0226!($globals);
    crate::section_0230::Put_each_of_tex_s_primitivies_into_the_hash_table_0230!($globals);
    crate::section_0238::Put_each_of_tex_s_primitivies_into_the_hash_table_0238!($globals);
    crate::section_0248::Put_each_of_tex_s_primitivies_into_the_hash_table_0248!($globals);
    crate::section_0265::Put_each_of_tex_s_primitivies_into_the_hash_table_0265!($globals);
    crate::section_0334::Put_each_of_tex_s_primitivies_into_the_hash_table_0334!($globals);
    crate::section_0376::Put_each_of_tex_s_primitivies_into_the_hash_table_0376!($globals);
    crate::section_0384::Put_each_of_tex_s_primitivies_into_the_hash_table_0384!($globals);
    crate::section_0411::Put_each_of_tex_s_primitivies_into_the_hash_table_0411!($globals);
    crate::section_0416::Put_each_of_tex_s_primitivies_into_the_hash_table_0416!($globals);
    crate::section_0468::Put_each_of_tex_s_primitivies_into_the_hash_table_0468!($globals);
    crate::section_0487::Put_each_of_tex_s_primitivies_into_the_hash_table_0487!($globals);
    crate::section_0491::Put_each_of_tex_s_primitivies_into_the_hash_table_0491!($globals);
    crate::section_0553::Put_each_of_tex_s_primitivies_into_the_hash_table_0553!($globals);
    crate::section_0780::Put_each_of_tex_s_primitivies_into_the_hash_table_0780!($globals);
    crate::section_0983::Put_each_of_tex_s_primitivies_into_the_hash_table_0983!($globals);
    crate::section_1052::Put_each_of_tex_s_primitivies_into_the_hash_table_1052!($globals);
    crate::section_1058::Put_each_of_tex_s_primitivies_into_the_hash_table_1058!($globals);
    crate::section_1071::Put_each_of_tex_s_primitivies_into_the_hash_table_1071!($globals);
    crate::section_1088::Put_each_of_tex_s_primitivies_into_the_hash_table_1088!($globals);
    crate::section_1107::Put_each_of_tex_s_primitivies_into_the_hash_table_1107!($globals);
    crate::section_1114::Put_each_of_tex_s_primitivies_into_the_hash_table_1114!($globals);
    crate::section_1141::Put_each_of_tex_s_primitivies_into_the_hash_table_1141!($globals);
    crate::section_1156::Put_each_of_tex_s_primitivies_into_the_hash_table_1156!($globals);
    crate::section_1169::Put_each_of_tex_s_primitivies_into_the_hash_table_1169!($globals);
    crate::section_1178::Put_each_of_tex_s_primitivies_into_the_hash_table_1178!($globals);
    crate::section_1188::Put_each_of_tex_s_primitivies_into_the_hash_table_1188!($globals);
    crate::section_1208::Put_each_of_tex_s_primitivies_into_the_hash_table_1208!($globals);
    crate::section_1219::Put_each_of_tex_s_primitivies_into_the_hash_table_1219!($globals);
    crate::section_1222::Put_each_of_tex_s_primitivies_into_the_hash_table_1222!($globals);
    crate::section_1230::Put_each_of_tex_s_primitivies_into_the_hash_table_1230!($globals);
    crate::section_1250::Put_each_of_tex_s_primitivies_into_the_hash_table_1250!($globals);
    crate::section_1254::Put_each_of_tex_s_primitivies_into_the_hash_table_1254!($globals);
    crate::section_1262::Put_each_of_tex_s_primitivies_into_the_hash_table_1262!($globals);
    crate::section_1272::Put_each_of_tex_s_primitivies_into_the_hash_table_1272!($globals);
    crate::section_1277::Put_each_of_tex_s_primitivies_into_the_hash_table_1277!($globals);
    crate::section_1286::Put_each_of_tex_s_primitivies_into_the_hash_table_1286!($globals);
    crate::section_1291::Put_each_of_tex_s_primitivies_into_the_hash_table_1291!($globals);
    crate::section_1344::Put_each_of_tex_s_primitivies_into_the_hash_table_1344!($globals);
    #[cfg(feature = "latex_support")]
    crate::latex_support::Put_each_of_tex_s_primitivies_into_the_hash_table_latex_support!(
        $globals
    );
}

// @!init procedure init_prim; {initialize all the primitives}
/// initialize all the primitives
#[cfg(feature = "initex")]
pub(crate) fn init_prim(globals: &mut TeXGlobals) {
    // begin no_new_control_sequence:=false;
    globals.no_new_control_sequence = false;
    // @<Put each...@>;
    Put_each_of_TeX_s_primitivies_into_the_hash_table!(globals);
    // no_new_control_sequence:=true;
    globals.no_new_control_sequence = true;
    // end;
    // tini
}

use crate::section_0004::TeXGlobals;
use linkme::distributed_slice;
