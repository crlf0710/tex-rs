//! ` `
//! During this loop, |p=link(q)| and there are |n| items preceding |p|.

// @<Prune the current list, if necessary...@>=
pub(crate) macro Prune_the_current_list__if_necessary__until_it_contains_only_char_node__kern_node__hlist_node__vlist_node__rule_node__and_ligature_node_items__set_n_to_the_length_of_the_list__and_set_q_to_the_list_s_tail($globals:expr, $p:expr, $q:expr, $n:expr) {{
    crate::region_forward_label! {
        |'done|
        {
            // q:=head; p:=link(q); n:=0;
            $q = head!($globals);
            $p = link!($globals, $q);
            $n = 0;
            // while p<>null do
            while $p != null {
                // begin if not is_char_node(p) then if type(p)>rule_node then
                //   if type(p)<>kern_node then if type(p)<>ligature_node then
                if !is_char_node!($globals, $p)
                    && r#type!($globals, $p) > rule_node
                    && r#type!($globals, $p) != kern_node
                    && r#type!($globals, $p) != ligature_node
                {
                    // begin print_err("Improper discretionary list");
                    print_err!($globals, crate::strpool_str!("Improper discretionary list"));
                    // @.Improper discretionary list@>
                    // help1("Discretionary lists must contain only boxes and kerns.");@/
                    help1!(
                        $globals,
                        crate::strpool_str!("Discretionary lists must contain only boxes and kerns.")
                    );
                    // error;
                    error($globals)?;
                    // begin_diagnostic;
                    begin_diagnostic($globals);
                    // print_nl("The following discretionary sublist has been deleted:");
                    print_nl($globals, crate::strpool_str!("The following discretionary sublist has been deleted:"));
                    // @.The following...deleted@>
                    // show_box(p);
                    show_box($globals, $p);
                    // end_diagnostic(true);
                    end_diagnostic($globals, true);
                    // flush_node_list(p); link(q):=null; goto done;
                    flush_node_list($globals, $p)?;
                    link!($globals, $q) = null;
                    crate::goto_forward_label!('done)
                    // end;
                }
                // q:=p; p:=link(q); incr(n);
                $q = $p;
                $p = link!($globals, $q);
                incr!($n);
                // end;
            }
        }
        // done:
        'done <-
    };
    use crate::section_0016::incr;
    use crate::section_0062::print_nl;
    use crate::section_0073::print_err;
    use crate::section_0079::help1;
    use crate::section_0082::error;
    use crate::section_0115::null;
    use crate::section_0118::link;
    use crate::section_0133::r#type;
    use crate::section_0134::is_char_node;
    use crate::section_0138::rule_node;
    use crate::section_0143::ligature_node;
    use crate::section_0155::kern_node;
    use crate::section_0198::show_box;
    use crate::section_0202::flush_node_list;
    use crate::section_0213::head;
    use crate::section_0245::begin_diagnostic;
    use crate::section_0245::end_diagnostic;
}}
