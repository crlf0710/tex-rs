//! ` `

// @<Set the glue in all the unset...@>=
pub(crate) macro Set_the_glue_in_all_the_unset_boxes_of_the_current_list($globals:expr, $p:expr, $o:expr) {{
    /// registers for the list operations
    let (mut q, mut s);
    // q:=link(head); s:=head;
    q = link!($globals, head!($globals));
    s = head!($globals);
    // while q<>null do
    while q != null {
        // begin if not is_char_node(q) then
        if !is_char_node!($globals, q) {
            // if type(q)=unset_node then
            if r#type!($globals, q) == unset_node {
                // @<Set the unset box |q| and the unset boxes in it@>
                crate::section_0807::Set_the_unset_box_q_and_the_unset_boxes_in_it!(
                    $globals, $p, q, $o
                );
            }
            // else if type(q)=rule_node then
            else if r#type!($globals, q) == rule_node {
                // @<Make the running dimensions in rule |q| extend to the
                //   boundaries of the alignment@>;
                todo!("Make the running dimensions");
            }
        }
        // s:=q; q:=link(q);
        s = q;
        q = link!($globals, q);
        // end
    }
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0138::rule_node;
    use crate::section_0159::unset_node;
    use crate::section_0213::head;
}}
