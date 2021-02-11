//! @ When we create an active node, we also create the corresponding
//! passive node.
//
// @<Insert a new active node from |best_place[fit_class]| to |cur_p|@>=
macro_rules! Insert_a_new_active_node_from_best_place_fit_class_to_cur_p {
    ($globals:expr, $r:expr, $prev_r:expr, $break_type:expr, $fit_class:expr) => {{
        /// points to a new node being created
        let mut q: pointer;
        // begin q:=get_node(passive_node_size);
        q = get_node($globals, passive_node_size.into())?;
        // link(q):=passive; passive:=q; cur_break(q):=cur_p;
        link!($globals, q) = $globals.passive;
        $globals.passive = q;
        cur_break!($globals, q) = $globals.cur_p;
        // @!stat incr(pass_number); serial(q):=pass_number;@+tats@;@/
        region_stat! {
            incr!($globals.pass_number);
            serial!($globals, q) = $globals.pass_number;
        }
        // prev_break(q):=best_place[fit_class];@/
        prev_break!($globals, q) = $globals.best_place[$fit_class];
        // q:=get_node(active_node_size); break_node(q):=passive;
        q = get_node($globals, active_node_size.into())?;
        break_node!($globals, q) = $globals.passive;
        // line_number(q):=best_pl_line[fit_class]+1;
        line_number!($globals, q) = $globals.best_pl_line[$fit_class] + 1;
        // fitness(q):=fit_class; type(q):=break_type;
        fitness!($globals, q) = $fit_class.get();
        r#type!($globals, q) = $break_type.get();
        // total_demerits(q):=minimal_demerits[fit_class];
        total_demerits!($globals, q) = $globals.minimal_demerits[$fit_class];
        // link(q):=r; link(prev_r):=q; prev_r:=q;
        link!($globals, q) = $r;
        link!($globals, $prev_r) = q;
        $prev_r = q;
        // @!stat if tracing_paragraphs>0 then
        region_stat! {
            if tracing_paragraphs!($globals) > 0 {
                // @<Print a symbolic description of the new break node@>;
                todo!("print a symbolic description");
            }
            // tats@;@/
        }
        // end
        use crate::section_0125::get_node;
        use crate::section_0819::active_node_size;
        use crate::section_0821::passive_node_size;
    }}
}
