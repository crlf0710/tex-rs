//! @ We take note of the value of \.{\\skip} |n| and the height plus depth
//! of \.{\\box}~|n| only when the first \.{\\insert}~|n| node is
//! encountered for a new page. A user who changes the contents of \.{\\box}~|n|
//! after that first \.{\\insert}~|n| had better be either extremely careful
//! or extremely lucky, or both.
//
// @<Create a page insertion node...@>=
pub(crate) macro Create_a_page_insertion_node_with_subtype_r_eq_qi_n__and_include_the_glue_correction_for_box_n_in_the_current_page_state($globals:expr, $r:expr, $n:expr) {{
    /// nodes being examined
    let mut q;
    /// sizes used for insertion calculations
    let h;
    // begin q:=get_node(page_ins_node_size); link(q):=link(r); link(r):=q; r:=q;
    q = get_node($globals, page_ins_node_size as _)?;
    link!($globals, q) = link!($globals, $r);
    link!($globals, $r) = q;
    $r = q;
    // subtype(r):=qi(n); type(r):=inserting; ensure_vbox(n);
    subtype!($globals, $r) = qi!($n);
    r#type!($globals, $r) = page_ins_node_type::inserting as _;
    ensure_vbox($globals, $n)?;
    // if box(n)=null then height(r):=0
    if r#box!($globals, $n) == null {
        height!($globals, $r) = scaled::zero();
    }
    // else height(r):=height(box(n))+depth(box(n));
    else {
        height!($globals, $r) =
            height!($globals, r#box!($globals, $n)) + depth!($globals, r#box!($globals, $n));
    }
    // best_ins_ptr(r):=null;@/
    best_ins_ptr!($globals, $r) = null;
    // q:=skip(n);
    q = skip!($globals, $n);
    // if count(n)=1000 then h:=height(r)
    if count!($globals, $n) == 1000 {
        h = height!($globals, $r);
    }
    // else h:=x_over_n(height(r),1000)*count(n);
    else {
        h = scaled::new_from_inner(
            x_over_n($globals, height!($globals, $r), 1000).inner() * count!($globals, $n),
        );
    }
    // page_goal:=page_goal-h-width(q);@/
    page_goal!($globals) = page_goal!($globals) - h - width!($globals, q);
    // page_so_far[2+stretch_order(q)]:=@|page_so_far[2+stretch_order(q)]+stretch(q);@/
    $globals.page_so_far[2 + stretch_order!($globals, q) as usize] += stretch!($globals, q);
    // page_shrink:=page_shrink+shrink(q);
    page_shrink!($globals) += shrink!($globals, q);
    // if (shrink_order(q)<>normal)and(shrink(q)<>0) then
    if shrink_order!($globals, q) != glue_ord::normal as _ && shrink!($globals, q) != scaled::zero()
    {
        todo!("Report error");
        // begin print_err("Infinite glue shrinkage inserted from "); print_esc("skip");
        // @.Infinite glue shrinkage...@>
        // print_int(n);
        // help3("The correction glue for page breaking with insertions")@/
        //   ("must have finite shrinkability. But you may proceed,")@/
        //   ("since the offensive shrinkability has been made finite.");
        // error;
        // end;
    }
    // end
    use crate::section_0101::scaled;
    use crate::section_0106::x_over_n;
    use crate::section_0112::qi;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0125::get_node;
    use crate::section_0133::r#type;
    use crate::section_0133::subtype;
    use crate::section_0135::depth;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0150::glue_ord;
    use crate::section_0150::shrink;
    use crate::section_0150::shrink_order;
    use crate::section_0150::stretch;
    use crate::section_0150::stretch_order;
    use crate::section_0224::skip;
    use crate::section_0230::r#box;
    use crate::section_0236::count;
    use crate::section_0981::best_ins_ptr;
    use crate::section_0981::page_ins_node_size;
    use crate::section_0981::page_ins_node_type;
    use crate::section_0982::page_goal;
    use crate::section_0982::page_shrink;
    use crate::section_0993::ensure_vbox;
}}
