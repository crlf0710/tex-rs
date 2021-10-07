//! ` `
// @d wrap_lig(#)==if ligature_present then
pub(crate) macro wrap_lig($globals:expr, $v:expr, $t:expr) {{
    if $globals.ligature_present {
        /// temporary register for list manipulation
        let p: pointer;

        // begin p:=new_ligature(hf,cur_l,link(cur_q));
        p = new_ligature(
            $globals,
            $globals.hf,
            ASCII_code::from($globals.cur_l as integer),
            link!($globals, $globals.cur_q),
        )?;
        // if lft_hit then
        if $globals.lft_hit {
            // begin subtype(p):=2; lft_hit:=false;
            subtype!($globals, p) = 2;
            $globals.lft_hit = false;
            // end;
        }
        // if # then if lig_stack=null then
        if $v && $globals.lig_stack == null {
            // begin incr(subtype(p)); rt_hit:=false;
            incr!(subtype!($globals, p));
            $globals.rt_hit = false;
            // end;
        }
        // link(cur_q):=p; t:=p; ligature_present:=false;
        link!($globals, $globals.cur_q) = p;
        $t = p;
        $globals.ligature_present = false;
        // end
    }
    use crate::pascal::integer;
    use crate::section_0016::incr;
    use crate::section_0018::ASCII_code;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0133::subtype;
    use crate::section_0144::new_ligature;
}}

// @d pop_lig_stack==begin if lig_ptr(lig_stack)>null then
pub(crate) macro pop_lig_stack($globals:expr, $t:expr, $j:expr, $n:expr, $bchar:expr, $hchar:expr, $cur_rh:expr) {{
    if lig_ptr!($globals, $globals.lig_stack) > null {
        // begin link(t):=lig_ptr(lig_stack); {this is a charnode for |hu[j+1]|}
        /// this is a charnode for `hu[j+1]`
        const _: () = ();
        link!($globals, $t) = lig_ptr!($globals, $globals.lig_stack);
        // t:=link(t); incr(j);
        $t = link!($globals, $t);
        incr!($j);
        // end;
    }
    /// temporary register for list manipulation
    let p: pointer;
    // p:=lig_stack; lig_stack:=link(p); free_node(p,small_node_size);
    p = $globals.lig_stack;
    $globals.lig_stack = link!($globals, p);
    free_node($globals, p, small_node_size as _);
    // if lig_stack=null then set_cur_r@+else cur_r:=character(lig_stack);
    if $globals.lig_stack == null {
        set_cur_r!($globals, $j, $n, $bchar, $hchar, $cur_rh);
    } else {
        $globals.cur_r = character!($globals, $globals.lig_stack).numeric_value() as _;
        /// if |lig_stack| isn't |null| we have |cur_rh=non_char|
        const _: () = ();
    }
    // end {if |lig_stack| isn't |null| we have |cur_rh=non_char|}
    use crate::section_0016::incr;
    use crate::section_0115::null;
    use crate::section_0115::pointer;
    use crate::section_0118::link;
    use crate::section_0130::free_node;
    use crate::section_0134::character;
    use crate::section_0141::small_node_size;
    use crate::section_0143::lig_ptr;
    use crate::section_0908::set_cur_r;
}}

// @<Append a ligature and/or kern to the translation...@>=
pub(crate) macro Append_a_ligature_and_or_kern_to_the_translation__goto_continue_if_the_stack_of_inserted_ligatures_is_nonempty($globals:expr, $t:expr, $w:expr, $j:expr, $n:expr, $bchar:expr, $hchar:expr, $cur_rh:expr, $lbl_continue:lifetime) {{
    // wrap_lig(rt_hit);
    wrap_lig!($globals, $globals.rt_hit, $t);
    // if w<>0 then
    if $w != scaled::zero() {
        // begin link(t):=new_kern(w); t:=link(t); w:=0;
        link!($globals, $t) = new_kern($globals, $w)?;
        $t = link!($globals, $t);
        $w = scaled::zero();
        // end;
    }
    // if lig_stack>null then
    if $globals.lig_stack > null {
        // begin cur_q:=t; cur_l:=character(lig_stack); ligature_present:=true;
        $globals.cur_q = $t;
        $globals.cur_l = character!($globals, $globals.lig_stack).numeric_value() as _;
        $globals.ligature_present = true;
        // pop_lig_stack; goto continue;
        pop_lig_stack!($globals, $t, $j, $n, $bchar, $hchar, $cur_rh);
        crate::goto_backward_label!($lbl_continue);
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0134::character;
    use crate::section_0156::new_kern;
}}
