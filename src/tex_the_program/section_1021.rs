//! ` `

// @<Wrap up the box specified by node |r|, splitting node |p| if...@>=
pub(crate) macro Wrap_up_the_box_specified_by_node_r__splitting_node_p_if_called_for__set_wait_to_true_if_node_p_holds_a_remainder_after_splitting($globals:expr, $p:expr, $r:expr) {{
    // begin if type(r)=split_up then
    if r#type!($globals, $r) == page_ins_node_type::split_up as _ {
        // if (broken_ins(r)=p)and(broken_ptr(r)<>null) then
        if broken_ins!($globals, $r) == $p && broken_ptr!($globals, $r) != null {
            // begin while link(s)<>broken_ptr(r) do s:=link(s);
            // link(s):=null;
            // split_top_skip:=split_top_ptr(p);
            // ins_ptr(p):=prune_page_top(broken_ptr(r));
            // if ins_ptr(p)<>null then
            //   begin temp_ptr:=vpack(ins_ptr(p),natural);
            //   height(p):=height(temp_ptr)+depth(temp_ptr);
            //   free_node(temp_ptr,box_node_size); wait:=true;
            //   end;
            todo!("split_up wrap up");
        }
        // end;
    }

    /// insertion box number
    let n;
    // best_ins_ptr(r):=null;
    best_ins_ptr!($globals, $r) = null;
    // n:=qo(subtype(r));
    n = qo!(subtype!($globals, $r));
    // temp_ptr:=list_ptr(box(n));
    $globals.temp_ptr = list_ptr!($globals, r#box!($globals, n));
    // free_node(box(n),box_node_size);
    free_node($globals, r#box!($globals, n), box_node_size as _);
    // box(n):=vpack(temp_ptr,natural);
    r#box!($globals, n) = vpack($globals, $globals.temp_ptr, natural0!(), natural1!())?;
    // end

    use crate::section_0112::qo;
    use crate::section_0115::null;
    use crate::section_0130::free_node;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0135::box_node_size;
    use crate::section_0135::list_ptr;
    use crate::section_0230::r#box;
    use crate::section_0644::natural0;
    use crate::section_0644::natural1;
    use crate::section_0668::vpack;
    use crate::section_0981::best_ins_ptr;
    use crate::section_0981::broken_ins;
    use crate::section_0981::broken_ptr;
    use crate::section_0981::page_ins_node_type;
}}
