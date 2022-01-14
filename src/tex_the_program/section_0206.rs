//! ` `
// @<Case statement to copy...@>=
pub(crate) macro Case_statement_to_copy_different_types_and_set_words_to_the_number_of_initial_words_not_yet_copied($globals:expr, $p:expr, $r:expr, $words:expr) {{
    // case type(p) of
    let type_p = r#type!($globals, $p);
    // hlist_node,vlist_node,unset_node: begin r:=get_node(box_node_size);
    if type_p == hlist_node || type_p == vlist_node || type_p == unset_node {
        // mem[r+6]:=mem[p+6]; mem[r+5]:=mem[p+5]; {copy the last two words}
        // list_ptr(r):=copy_node_list(list_ptr(p)); {this affects |mem[r+5]|}
        // words:=5;
        // end;
        todo!("list");
    }
    // rule_node: begin r:=get_node(rule_node_size); words:=rule_node_size;
    else if type_p == rule_node {
        $r = get_node($globals, rule_node_size as _)?;
        $words = rule_node_size as _;
        // end;
    }
    // ins_node: begin r:=get_node(ins_node_size); mem[r+4]:=mem[p+4];
    //   add_glue_ref(split_top_ptr(p));
    //   ins_ptr(r):=copy_node_list(ins_ptr(p)); {this affects |mem[r+4]|}
    //   words:=ins_node_size-1;
    //   end;
    // whatsit_node:@<Make a partial copy of the whatsit node |p| and make |r|
    //   point to it; set |words| to the number of initial words not yet copied@>;
    // glue_node: begin r:=get_node(small_node_size); add_glue_ref(glue_ptr(p));
    //   glue_ptr(r):=glue_ptr(p); leader_ptr(r):=copy_node_list(leader_ptr(p));
    //   end;
    // kern_node,math_node,penalty_node: begin r:=get_node(small_node_size);
    //   words:=small_node_size;
    //   end;
    // ligature_node: begin r:=get_node(small_node_size);
    //   mem[lig_char(r)]:=mem[lig_char(p)]; {copy |font| and |character|}
    //   lig_ptr(r):=copy_node_list(lig_ptr(p));
    //   end;
    // disc_node: begin r:=get_node(small_node_size);
    //   pre_break(r):=copy_node_list(pre_break(p));
    //   post_break(r):=copy_node_list(post_break(p));
    //   end;
    // mark_node: begin r:=get_node(small_node_size); add_token_ref(mark_ptr(p));
    //   words:=small_node_size;
    //   end;
    // adjust_node: begin r:=get_node(small_node_size);
    //   adjust_ptr(r):=copy_node_list(adjust_ptr(p));
    //   end; {|words=1=small_node_size-1|}
    // othercases confusion("copying")
    else {
        crate::trace_error_expr!("type(p)={}", type_p);
        confusion($globals, crate::strpool_str!("copying"))?;
        // @:this can't happen copying}{\quad copying@>
    }
    // endcases
    use crate::section_0095::confusion;
    use crate::section_0125::get_node;
    use crate::section_0133::r#type;
    use crate::section_0135::hlist_node;
    use crate::section_0137::vlist_node;
    use crate::section_0138::rule_node;
    use crate::section_0138::rule_node_size;
    use crate::section_0159::unset_node;
}}
