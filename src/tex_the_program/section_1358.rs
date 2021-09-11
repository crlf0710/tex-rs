//! ` `

// @<Wipe out the whatsit...@>=
pub(crate) macro Wipe_out_the_whatsit_node_p_and_goto_done($globals:expr, $p:expr, $lbl_done:lifetime) {{
    let p = $p;
    let subtype_p = subtype!($globals, p);
    // begin case subtype(p) of
    // open_node: free_node(p,open_node_size);
    if subtype_p == open_node {
        free_node($globals, p, open_node_size as _);
    }
    // write_node,special_node: begin delete_token_ref(write_tokens(p));
    //   free_node(p,write_node_size); goto done;
    //   end;
    else if subtype_p == write_node || subtype_p == special_node {
        delete_token_ref($globals, write_tokens!($globals, p));
        free_node($globals, p, write_node_size as _);
        crate::goto_forward_label!($lbl_done);
    }
    // close_node,language_node: free_node(p,small_node_size);
    else if subtype_p == close_node || subtype_p == language_node {
        free_node($globals, p, small_node_size as _);
    }
    // othercases confusion("ext3")
    else {
        confusion($globals, crate::strpool_str!("ext3"))?;
    }
    // @:this can't happen ext3}{\quad ext3@>
    // endcases;@/
    // goto done;
    crate::goto_forward_label!($lbl_done);
    // end

    use crate::section_0095::confusion;
    use crate::section_0130::free_node;
    use crate::section_0133::subtype;
    use crate::section_0141::small_node_size;
    use crate::section_0200::delete_token_ref;
    use crate::section_1341::close_node;
    use crate::section_1341::language_node;
    use crate::section_1341::open_node;
    use crate::section_1341::open_node_size;
    use crate::section_1341::special_node;
    use crate::section_1341::write_node;
    use crate::section_1341::write_node_size;
    use crate::section_1341::write_tokens;
}}
