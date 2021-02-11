//! @ The job of reversing links in a list is conveniently regarded as the job
//! of taking items off one stack and putting them on another. In this case we
//! take them off a stack pointed to by |q| and having |prev_break| fields;
//! we put them on a stack pointed to by |cur_p| and having |next_break| fields.
//! Node |r| is the passive node being moved from stack to stack.
//
// @<Reverse the links of the relevant passive nodes...@>=
macro_rules! Reverse_the_links_of_the_relevant_passive_nodes__setting_cur_p_to_the_first_breakpoint {
    ($globals:expr) => {{
        /// temporary registers for list manipulation
        let (mut q, mut r): (pointer, pointer);

        // q:=break_node(best_bet); cur_p:=null;
        q = break_node!($globals, $globals.best_bet);
        $globals.cur_p = null;
        // repeat r:=q; q:=prev_break(q); next_break(r):=cur_p; cur_p:=r;
        loop {
            r = q;
            q = prev_break!($globals, q);
            next_break!($globals, r) = $globals.cur_p;
            $globals.cur_p = r;
            // until q=null
            if q == null {
                break;
            }
        }
        use crate::section_0115::pointer;
    }}
}
