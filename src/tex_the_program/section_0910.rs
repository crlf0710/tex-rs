//! ` `
// @d wrap_lig(#)==if ligature_present then
macro wrap_lig($globals:expr, $v:expr) {{
    if $globals.ligature_present {
        // begin p:=new_ligature(hf,cur_l,link(cur_q));
        // if lft_hit then
        //   begin subtype(p):=2; lft_hit:=false;
        //   end;
        // if # then if lig_stack=null then
        //   begin incr(subtype(p)); rt_hit:=false;
        //   end;
        // link(cur_q):=p; t:=p; ligature_present:=false;
        // end
        todo!("wrap_lig");
    }
}}
// @d pop_lig_stack==begin if lig_ptr(lig_stack)>null then
//     begin link(t):=lig_ptr(lig_stack); {this is a charnode for |hu[j+1]|}
//     t:=link(t); incr(j);
//     end;
//   p:=lig_stack; lig_stack:=link(p); free_node(p,small_node_size);
//   if lig_stack=null then set_cur_r@+else cur_r:=character(lig_stack);
//   end {if |lig_stack| isn't |null| we have |cur_rh=non_char|}
//
// @<Append a ligature and/or kern to the translation...@>=
pub(crate) macro Append_a_ligature_and_or_kern_to_the_translation__goto_continue_if_the_stack_of_inserted_ligatures_is_nonempty($globals:expr, $w:expr) {{
    // wrap_lig(rt_hit);
    wrap_lig!($globals, $globals.rt_hit);
    // if w<>0 then
    if $w != scaled::zero() {
        // begin link(t):=new_kern(w); t:=link(t); w:=0;
        // end;
        todo!("w != 0");
    }
    // if lig_stack>null then
    if $globals.lig_stack > null {
        // begin cur_q:=t; cur_l:=character(lig_stack); ligature_present:=true;
        // pop_lig_stack; goto continue;
        // end
        todo!("lig_stack > null");
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
}}
