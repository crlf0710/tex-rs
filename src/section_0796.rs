//! ` `
// @<Package an unset...@>=
macro_rules! Package_an_unset_box_for_the_current_column_and_record_its_width {
    ($globals:expr) => {{
        /// a new unset box
        let u: pointer;
        /// natural width
        let w: scaled;
        /// span counter
        let n: halfword;
        /// order of infinity
        let mut o: glue_ord;
        // begin if mode=-hmode then
        if mode!($globals) == -hmode {
            todo!("mode = -hmode");
        // begin adjust_tail:=cur_tail; u:=hpack(link(head),natural); w:=width(u);
        // cur_tail:=adjust_tail; adjust_tail:=null;
        // end
        }
        // else  begin u:=vpackage(link(head),natural,0); w:=height(u);
        else {
            u = vpackage(
                $globals,
                link!($globals, head!($globals)),
                natural0!(),
                natural1!(),
                scaled::zero(),
            )?;
            w = height!($globals, u);
            // end;
        }
        // n:=min_quarterword; {this represents a span count of 1}
        /// this represents a span count of 1
        const _: () = ();
        n = min_quarterword as _;
        // if cur_span<>cur_align then @<Update width entry for spanned columns@>
        if $globals.cur_span != $globals.cur_align {
            todo!("update width entry");
        }
        // else if w>width(cur_align) then width(cur_align):=w;
        else if w > width!($globals, $globals.cur_align) {
            width!($globals, $globals.cur_align) = w;
        }
        // type(u):=unset_node; span_count(u):=n;@/
        r#type!($globals, u) = unset_node;
        span_count!($globals, u) = n as _;
        // @<Determine the stretch order@>;
        Determine_the_stretch_order!($globals, o);
        // glue_order(u):=o; glue_stretch(u):=total_stretch[o];@/
        glue_order!($globals, u) = o as _;
        glue_stretch!($globals, u) = $globals.total_stretch[o];
        // @<Determine the shrink order@>;
        Determine_the_shrink_order!($globals, o);
        // glue_sign(u):=o; glue_shrink(u):=total_shrink[o];@/
        glue_sign!($globals, u) = o as _;
        glue_shrink!($globals, u) = $globals.total_shrink[o];
        // pop_nest; link(tail):=u; tail:=u;
        pop_nest($globals);
        link!($globals, tail!($globals)) = u;
        tail!($globals) = u;
        // end
        use crate::section_0101::scaled;
        use crate::section_0110::min_quarterword;
        use crate::section_0113::halfword;
        use crate::section_0150::glue_ord;
        use crate::section_0159::unset_node;
        use crate::section_0211::hmode;
        use crate::section_0217::pop_nest;
        use crate::section_0668::vpackage;
    }};
}
