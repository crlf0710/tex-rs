//! ` `
// @<Clean...@>=
macro_rules! Clean_up_the_memory_by_removing_the_break_nodes {
    ($globals:expr) => {{
        let mut q: pointer;
        // q:=link(active);
        q = link!($globals, active);
        // while q<>last_active do
        while q != last_active!() {
            // begin cur_p:=link(q);
            $globals.cur_p = link!($globals, q);
            // if type(q)=delta_node then free_node(q,delta_node_size)
            if r#type!($globals, q) == delta_node {
                free_node($globals, q, delta_node_size as _);
            }
            // else free_node(q,active_node_size);
            else {
                free_node($globals, q, active_node_size as _);
            }
            // q:=cur_p;
            q = $globals.cur_p;
            // end;
        }
        // q:=passive;
        q = $globals.passive;
        // while q<>null do
        while q != null {
            todo!("passive");
            // begin cur_p:=link(q);
            // free_node(q,passive_node_size);
            // q:=cur_p;
            // end
        }
        use crate::section_0115::pointer;
        use crate::section_0115::null;
        use crate::section_0130::free_node;
        use crate::section_0162::active;
        use crate::section_0819::active_node_size;
        use crate::section_0822::delta_node_size;
        use crate::section_0822::delta_node;
    }}
}
