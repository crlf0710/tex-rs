//! @ The token list |omit_template| just referred to is a constant token
//! list that contains the special control sequence \.{\\endtemplate} only.
//
// @<Initialize the special...@>=
pub(crate) macro Initialize_the_special_list_heads_and_constant_nodes_0790($globals:expr) {{
    // info(omit_template):=end_template_token; {|link(omit_template)=null|}
    info_tok_assign!(
        $globals,
        omit_template,
        cur_tok_type::new(end_template_token)
    );
    /// `link(omit_template)=null`
    const _: () = ();

    use crate::section_0118::info_tok_assign;
    use crate::section_0162::omit_template;
    use crate::section_0297::cur_tok_type;
    use crate::section_0780::end_template_token;
}}
