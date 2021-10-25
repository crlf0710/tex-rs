//! ` `
//!
//!  Primitive math operators like \.{\\mathop} and \.{\\underline} are given
//! the command code |math_comp|, supplemented by the noad type that they
//! generate.
//
// @<Put each...@>=
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1156($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("mathord",math_comp,ord_noad);
    primitive(
        globals,
        crate::strpool_str!("mathord"),
        math_comp,
        ord_noad as _,
    );
    // @!@:math_ord_}{\.{\\mathord} primitive@>
    // primitive("mathop",math_comp,op_noad);
    primitive(
        globals,
        crate::strpool_str!("mathop"),
        math_comp,
        op_noad as _,
    );
    // @!@:math_op_}{\.{\\mathop} primitive@>
    // primitive("mathbin",math_comp,bin_noad);
    primitive(
        globals,
        crate::strpool_str!("mathbin"),
        math_comp,
        bin_noad as _,
    );
    // @!@:math_bin_}{\.{\\mathbin} primitive@>
    // primitive("mathrel",math_comp,rel_noad);
    primitive(
        globals,
        crate::strpool_str!("mathrel"),
        math_comp,
        rel_noad as _,
    );
    // @!@:math_rel_}{\.{\\mathrel} primitive@>
    // primitive("mathopen",math_comp,open_noad);
    primitive(
        globals,
        crate::strpool_str!("mathopen"),
        math_comp,
        open_noad as _,
    );
    // @!@:math_open_}{\.{\\mathopen} primitive@>
    // primitive("mathclose",math_comp,close_noad);
    primitive(
        globals,
        crate::strpool_str!("mathclose"),
        math_comp,
        close_noad as _,
    );
    // @!@:math_close_}{\.{\\mathclose} primitive@>
    // primitive("mathpunct",math_comp,punct_noad);
    primitive(
        globals,
        crate::strpool_str!("mathpunct"),
        math_comp,
        punct_noad as _,
    );
    // @!@:math_punct_}{\.{\\mathpunct} primitive@>
    // primitive("mathinner",math_comp,inner_noad);
    primitive(
        globals,
        crate::strpool_str!("mathinner"),
        math_comp,
        inner_noad as _,
    );
    // @!@:math_inner_}{\.{\\mathinner} primitive@>
    // primitive("underline",math_comp,under_noad);
    primitive(
        globals,
        crate::strpool_str!("underline"),
        math_comp,
        under_noad as _,
    );
    // @!@:underline_}{\.{\\underline} primitive@>
    // primitive("overline",math_comp,over_noad);@/
    primitive(
        globals,
        crate::strpool_str!("overline"),
        math_comp,
        over_noad as _,
    );
    // @!@:overline_}{\.{\\overline} primitive@>
    // primitive("displaylimits",limit_switch,normal);
    primitive(
        globals,
        crate::strpool_str!("displaylimits"),
        limit_switch,
        op_noad_subtype::normal as _,
    );
    // @!@:display_limits_}{\.{\\displaylimits} primitive@>
    // primitive("limits",limit_switch,limits);
    primitive(
        globals,
        crate::strpool_str!("limits"),
        limit_switch,
        op_noad_subtype::limits as _,
    );
    // @!@:limits_}{\.{\\limits} primitive@>
    // primitive("nolimits",limit_switch,no_limits);
    primitive(
        globals,
        crate::strpool_str!("nolimits"),
        limit_switch,
        op_noad_subtype::no_limits as _,
    );
    // @!@:no_limits_}{\.{\\nolimits} primitive@>
    use crate::section_0208::*;
    use crate::section_0264::primitive;
    use crate::section_0682::*;
    use crate::section_0687::*;
}}
