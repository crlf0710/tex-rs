//! ` `
// @<Case statement to copy...@>=
pub(crate) macro Case_statement_to_copy_different_types_and_set_words_to_the_number_of_initial_words_not_yet_copied($globals:expr, $p:expr, $r:expr, $words:expr) {{
    // case type(p) of
    let type_p = r#type!($globals, $p);
    // hlist_node,vlist_node,unset_node: begin r:=get_node(box_node_size);
    if type_p == hlist_node || type_p == vlist_node || type_p == unset_node {
        $r = get_node($globals, box_node_size as _)?;
        // mem[r+6]:=mem[p+6]; mem[r+5]:=mem[p+5]; {copy the last two words}
        /// copy the last two words
        const _: () = ();
        $globals.mem[$r + 6] = $globals.mem[$p + 6];
        $globals.mem[$r + 5] = $globals.mem[$p + 5];
        // list_ptr(r):=copy_node_list(list_ptr(p)); {this affects |mem[r+5]|}
        /// this affects `mem[r+5]`
        const _: () = ();
        list_ptr!($globals, $r) = copy_node_list($globals, list_ptr!($globals, $p))?;
        // words:=5;
        $words = 5;
        // end;
    }
    // rule_node: begin r:=get_node(rule_node_size); words:=rule_node_size;
    else if type_p == rule_node {
        $r = get_node($globals, rule_node_size as _)?;
        $words = rule_node_size as _;
        // end;
    }
    // ins_node: begin r:=get_node(ins_node_size); mem[r+4]:=mem[p+4];
    else if type_p == ins_node {
        $r = get_node($globals, ins_node_size as _)?;
        $globals.mem[$r + 4] = $globals.mem[$p + 4];
        // add_glue_ref(split_top_ptr(p));
        let split_top_ptr = split_top_ptr!($globals, $p);
        add_glue_ref!($globals, split_top_ptr);
        // ins_ptr(r):=copy_node_list(ins_ptr(p)); {this affects |mem[r+4]|}
        /// this affects `mem[r+4]`
        const _: () = ();
        ins_ptr!($globals, $r) = copy_node_list($globals, ins_ptr!($globals, $p))?;
        // words:=ins_node_size-1;
        $words = (ins_node_size - 1) as _;
        // end;
    }
    // whatsit_node:@<Make a partial copy of the whatsit node |p| and make |r|
    //   point to it; set |words| to the number of initial words not yet copied@>;
    else if type_p == whatsit_node {
        todo!("Make a partial copy");
    }
    // glue_node: begin r:=get_node(small_node_size); add_glue_ref(glue_ptr(p));
    else if type_p == glue_node {
        $r = get_node($globals, small_node_size as _)?;
        let glue_ptr_p = glue_ptr!($globals, $p);
        add_glue_ref!($globals, glue_ptr_p);
        // glue_ptr(r):=glue_ptr(p); leader_ptr(r):=copy_node_list(leader_ptr(p));
        glue_ptr!($globals, $r) = glue_ptr_p;
        leader_ptr!($globals, $r) = copy_node_list($globals, leader_ptr!($globals, $p))?;
        // end;
    }
    // kern_node,math_node,penalty_node: begin r:=get_node(small_node_size);
    else if type_p == kern_node || type_p == math_node || type_p == penalty_node {
        $r = get_node($globals, small_node_size as _)?;
        // words:=small_node_size;
        $words = small_node_size as _;
        // end;
    }
    // ligature_node: begin r:=get_node(small_node_size);
    else if type_p == ligature_node {
        $r = get_node($globals, small_node_size as _)?;
        // mem[lig_char(r)]:=mem[lig_char(p)]; {copy |font| and |character|}
        /// copy `font` and `character`
        const _: () = ();
        $globals.mem[lig_char!($r)] = $globals.mem[lig_char!($p)];
        // lig_ptr(r):=copy_node_list(lig_ptr(p));
        lig_ptr!($globals, $r) = copy_node_list($globals, lig_ptr!($globals, $p))?;
        // end;
    }
    // disc_node: begin r:=get_node(small_node_size);
    else if type_p == disc_node {
        $r = get_node($globals, small_node_size as _)?;
        // pre_break(r):=copy_node_list(pre_break(p));
        pre_break!($globals, $r) = copy_node_list($globals, pre_break!($globals, $p))?;
        // post_break(r):=copy_node_list(post_break(p));
        post_break!($globals, $r) = copy_node_list($globals, post_break!($globals, $p))?;
        // end;
    }
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
    use crate::section_0135::box_node_size;
    use crate::section_0135::hlist_node;
    use crate::section_0135::list_ptr;
    use crate::section_0137::vlist_node;
    use crate::section_0138::rule_node;
    use crate::section_0138::rule_node_size;
    use crate::section_0140::ins_node;
    use crate::section_0140::ins_node_size;
    use crate::section_0140::ins_ptr;
    use crate::section_0140::split_top_ptr;
    use crate::section_0141::small_node_size;
    use crate::section_0143::lig_char;
    use crate::section_0143::lig_ptr;
    use crate::section_0143::ligature_node;
    use crate::section_0145::disc_node;
    use crate::section_0145::post_break;
    use crate::section_0145::pre_break;
    use crate::section_0146::whatsit_node;
    use crate::section_0147::math_node;
    use crate::section_0149::glue_node;
    use crate::section_0149::glue_ptr;
    use crate::section_0149::leader_ptr;
    use crate::section_0155::kern_node;
    use crate::section_0157::penalty_node;
    use crate::section_0159::unset_node;
    use crate::section_0203::add_glue_ref;
    use crate::section_0204::copy_node_list;
}}
