//! ` `
// @<Show the status of the current page@>=
macro_rules! Show_the_status_of_the_current_page {
    ($globals:expr) => {{
        // if page_head<>page_tail then
        if page_head != $globals.page_tail {
            // begin print_nl("### current page:");
            print_nl($globals, strpool_str!("### current page:"));
            // if output_active then print(" (held over for next output)");
            if $globals.output_active {
                print(
                    $globals,
                    strpool_str!(" (held over for next output)").get() as _,
                );
            }
            // @.held over for next output@>
            // show_box(link(page_head));
            show_box($globals, link!($globals, page_head));
            // if page_contents>empty then
            if $globals.page_contents > page_contents_kind::empty {
                /// for showing the current page
                let (mut q, mut r): (pointer, pointer);
                // begin print_nl("total height "); print_totals;
                print_nl($globals, strpool_str!("total height "));
                print_totals($globals);
                // @:total_height}{\.{total height}@>
                // print_nl(" goal height "); print_scaled(page_goal);
                print_nl($globals, strpool_str!(" goal height "));
                print_scaled($globals, page_goal!($globals));
                // @.goal height@>
                // r:=link(page_ins_head);
                r = link!($globals, page_ins_head);
                // while r<>page_ins_head do
                while r != page_ins_head {
                    /// for showing the current page
                    let mut t: integer;
                    // begin print_ln; print_esc("insert"); t:=qo(subtype(r));
                    print_ln(make_globals_io_string_log_view!($globals));
                    print_esc($globals, strpool_str!("insert"));
                    t = subtype!($globals, r) as _;
                    // print_int(t); print(" adds ");
                    print_int($globals, t);
                    print($globals, strpool_str!(" adds ").get() as _);
                    // if count(t)=1000 then t:=height(r)
                    if count!($globals, t) == 1000 {
                        t = height!($globals, r).inner();
                    }
                    // else t:=x_over_n(height(r),1000)*count(t);
                    else {
                        t = x_over_n($globals, height!($globals, r), 1000).inner()
                            * count!($globals, t);
                    }
                    // print_scaled(t);
                    print_scaled($globals, scaled::new_from_inner(t));
                    // if type(r)=split_up then
                    if r#type!($globals, r) as integer == page_ins_node_subtype::split_up as integer
                    {
                        // begin q:=page_head; t:=0;
                        q = page_head;
                        t = 0;
                        // repeat q:=link(q);
                        loop {
                            q = link!($globals, q);
                            // if (type(q)=ins_node)and(subtype(q)=subtype(r)) then incr(t);
                            if r#type!($globals, q) == ins_node
                                && subtype!($globals, q) == subtype!($globals, r)
                            {
                                incr!(t);
                            }
                            // until q=broken_ins(r);
                            if q == broken_ins!($globals, r) {
                                break;
                            }
                        }
                        // print(", #"); print_int(t); print(" might split");
                        print($globals, strpool_str!(", #").get() as _);
                        print_int($globals, t);
                        print($globals, strpool_str!(" might split").get() as _);
                        // end;
                    }
                    // r:=link(r);
                    r = link!($globals, r);
                    // end;
                }
                // end;
            }
            // end
        }
        use crate::pascal::integer;
        use crate::section_0063::print_esc;
        use crate::section_0101::scaled;
        use crate::section_0103::print_scaled;
        use crate::section_0106::x_over_n;
        use crate::section_0115::pointer;
        use crate::section_0140::ins_node;
        use crate::section_0162::page_head;
        use crate::section_0162::page_ins_head;
        use crate::section_0980::page_contents_kind;
        use crate::section_0981::page_ins_node_subtype;
        use crate::section_0985::print_totals;
    }};
}
