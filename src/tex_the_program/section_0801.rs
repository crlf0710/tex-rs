//! ` `
//! It's time now to dismantle the preamble list and to compute the column
//! widths. Let $w_{ij}$ be the maximum of the natural widths of all entries
//! that span columns $i$ through $j$, inclusive. The alignrecord for column~$i$
//! contains $w_{ii}$ in its |width| field, and there is also a linked list of
//! the nonzero $w_{ij}$ for increasing $j$, accessible via the |info| field;
//! these span nodes contain the value $j-i+|min_quarterword|$ in their
//! |link| fields. The values of $w_{ii}$ were initialized to |null_flag|, which
//! we regard as $-\infty$.
//!
//! The final column widths are defined by the formula
//! $$w_j=\max_{1\L i\L j}\biggl( w_{ij}-\sum_{i\L k<j}(t_k+w_k)\biggr),$$
//! where $t_k$ is the natural width of the tabskip glue between columns
//! $k$ and~$k+1$. However, if $w_{ij}=-\infty$ for all |i| in the range
//! |1<=i<=j| (i.e., if every entry that involved column~|j| also involved
//! column~|j+1|), we let $w_j=0$, and we zero out the tabskip glue after
//! column~|j|.
//!
//! \TeX\ computes these values by using the following scheme: First $w_1=w_{11}$.
//! Then replace $w_{2j}$ by $\max(w_{2j},w_{1j}-t_1-w_1)$, for all $j>1$.
//! Then $w_2=w_{22}$. Then replace $w_{3j}$ by $\max(w_{3j},w_{2j}-t_2-w_2)$
//! for all $j>2$; and so on. If any $w_j$ turns out to be $-\infty$, its
//! value is changed to zero and so is the next tabskip.
//
// @<Go through the preamble list,...@>=
pub(crate) macro Go_through_the_preamble_list__determining_the_column_widths_and_changing_the_alignrecords_to_dummy_unset_boxes($globals:expr) {{
    /// registers for the list operations
    let (mut p, mut q);
    // q:=link(preamble);
    q = link!($globals, preamble!($globals));
    // repeat flush_list(u_part(q)); flush_list(v_part(q));
    loop {
        flush_list($globals, u_part!($globals, q) as _);
        flush_list($globals, v_part!($globals, q) as _);
        // p:=link(link(q));
        p = link!($globals, link!($globals, q));
        // if width(q)=null_flag then
        if width!($globals, q) == null_flag {
            // @<Nullify |width(q)| and the tabskip glue following this column@>;
            todo!("Nullify");
        }
        // if info(q)<>end_span then
        if info_inner!($globals, q) != end_span {
            // @<Merge the widths in the span nodes of |q| with those of |p|,
            //   destroying the span nodes of |q|@>;
            todo!("Merge");
        }
        // type(q):=unset_node; span_count(q):=min_quarterword; height(q):=0;
        r#type!($globals, q) = unset_node;
        span_count!($globals, q) = min_quarterword;
        height!($globals, q) = scaled::zero();
        // depth(q):=0; glue_order(q):=normal; glue_sign(q):=normal;
        depth!($globals, q) = scaled::zero();
        glue_order!($globals, q) = glue_ord::normal as _;
        glue_sign!($globals, q) = glue_sign::normal as _;
        // glue_stretch(q):=0; glue_shrink(q):=0; q:=p;
        glue_stretch!($globals, q) = scaled::zero();
        glue_shrink!($globals, q) = scaled::zero();
        q = p;
        // until q=null
        if q == null {
            break;
        }
    }
    use crate::section_0101::scaled;
    use crate::section_0110::min_quarterword;
    use crate::section_0115::null;
    use crate::section_0118::info_inner;
    use crate::section_0118::link;
    use crate::section_0123::flush_list;
    use crate::section_0133::r#type;
    use crate::section_0135::depth;
    use crate::section_0135::glue_order;
    use crate::section_0135::glue_sign;
    use crate::section_0135::height;
    use crate::section_0135::width;
    use crate::section_0138::null_flag;
    use crate::section_0150::glue_ord;
    use crate::section_0159::glue_shrink;
    use crate::section_0159::glue_stretch;
    use crate::section_0159::span_count;
    use crate::section_0159::unset_node;
    use crate::section_0162::end_span;
    use crate::section_0769::u_part;
    use crate::section_0769::v_part;
    use crate::section_0770::preamble;
}}
