//! @ Here is the code that will split a long footnote between pages, in an
//! emergency. The current situation deserves to be recapitulated: Node |p|
//! is an insertion into box |n|; the insertion will not fit, in its entirety,
//! either because it would make the total contents of box |n| greater than
//! \.{\\dimen} |n|, or because it would make the incremental amount of growth
//! |h| greater than the available space |delta|, or both. (This amount |h| has
//! been weighted by the insertion scaling factor, i.e., by \.{\\count} |n|
//! over 1000.) Now we will choose the best way to break the vlist of the
//! insertion, using the same criteria as in the \.{\\vsplit} operation.
//
// @<Find the best way to split the insertion...@>=
pub(crate) macro Find_the_best_way_to_split_the_insertion__and_change_type_r_to_split_up($globals:expr, $r:expr, $n:expr, $p:expr) {{
    /// nodes being examined
    let q;
    /// sizes used for insertion calculations
    let mut w;
    // begin if count(n)<=0 then w:=max_dimen
    if count!($globals, $n) <= 0 {
        w = scaled::new_from_inner(max_dimen);
    }
    // else  begin w:=page_goal-page_total-page_depth;
    else {
        w = page_goal!($globals) - page_total!($globals) - page_depth!($globals);
        // if count(n)<>1000 then w:=x_over_n(w,count(n))*1000;
        if count!($globals, $n) != 1000 {
            w = scaled::new_from_inner(x_over_n($globals, w, count!($globals, $n)).inner() * 1000);
        }
        // end;
    }
    // if w>dimen(n)-height(r) then w:=dimen(n)-height(r);
    if w > dimen!($globals, $n) - height!($globals, $r) {
        w = dimen!($globals, $n) - height!($globals, $r);
    }
    // q:=vert_break(ins_ptr(p),w,depth(p));
    q = vert_break($globals, ins_ptr!($globals, $p), w, depth!($globals, $p))?;
    // height(r):=height(r)+best_height_plus_depth;
    height!($globals, $r) += $globals.best_height_plus_depth;
    // @!stat if tracing_pages>0 then @<Display the insertion split cost@>;@+tats@;@/
    crate::region_stat! {
        if tracing_pages!($globals) > 0 {
            todo!("Display the insertion...");
        }
        use crate::section_0236::tracing_pages;
    }
    // if count(n)<>1000 then
    if count!($globals, $n) != 1000 {
        // best_height_plus_depth:=x_over_n(best_height_plus_depth,1000)*count(n);
        $globals.best_height_plus_depth = scaled::new_from_inner(
            x_over_n($globals, $globals.best_height_plus_depth, 1000).inner()
                * count!($globals, $n),
        );
    }
    // page_goal:=page_goal-best_height_plus_depth;
    page_goal!($globals) -= $globals.best_height_plus_depth;
    // type(r):=split_up; broken_ptr(r):=q; broken_ins(r):=p;
    r#type!($globals, $r) = page_ins_node_type::split_up as _;
    broken_ptr!($globals, $r) = q;
    broken_ins!($globals, $r) = $p;
    // if q=null then insert_penalties:=insert_penalties+eject_penalty
    if q == null {
        $globals.insert_penalties += eject_penalty;
    }
    // else if type(q)=penalty_node then insert_penalties:=insert_penalties+penalty(q);
    else if r#type!($globals, q) == penalty_node {
        $globals.insert_penalties += penalty!($globals, q);
        // end
    }
    use crate::section_0101::scaled;
    use crate::section_0106::x_over_n;
    use crate::section_0115::null;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0140::ins_ptr;
    use crate::section_0157::eject_penalty;
    use crate::section_0157::penalty;
    use crate::section_0157::penalty_node;
    use crate::section_0236::count;
    use crate::section_0247::dimen;
    use crate::section_0421::max_dimen;
    use crate::section_0970::vert_break;
    use crate::section_0981::broken_ins;
    use crate::section_0981::broken_ptr;
    use crate::section_0981::page_ins_node_type;
    use crate::section_0982::page_depth;
    use crate::section_0982::page_goal;
    use crate::section_0982::page_total;
}}
