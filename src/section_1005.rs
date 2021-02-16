//! ` `

// @<Check if node |p| is a new champion breakpoint; then \(if)...@>=
macro_rules! Check_if_node_p_is_a_new_champion_breakpoint__then_if_it_is_time_for_a_page_break__prepare_for_output__and_either_fire_up_the_user_s_output_routine_and_return_or_ship_out_the_page_and_goto_done {
    ($globals:expr, $p:expr, $pi:expr, $lbl_done:lifetime) => {{
        /// badness and cost of current page
        let (b, mut c): (integer, integer);
        // if pi<inf_penalty then
        if $pi < inf_penalty {
            // begin @<Compute the badness, |b|, of the current page,
            //   using |awful_bad| if the box is too full@>;
            Compute_the_badness_b__of_the_current_page__using_awful_bad_if_the_box_is_too_full!
                ($globals, b);
            // if b<awful_bad then
            if b < awful_bad {
                // if pi<=eject_penalty then c:=pi
                if $pi <= eject_penalty {
                    c = $pi;
                }
                // else  if b<inf_bad then c:=b+pi+insert_penalties
                else {
                    if b < inf_bad as integer {
                        c = b + $pi + $globals.insert_penalties;
                    }
                    // else c:=deplorable
                    else {
                        c = deplorable;
                    }
                }
            }
            // else c:=b;
            else {
                c = b;
            }
            // if insert_penalties>=10000 then c:=awful_bad;
            if $globals.insert_penalties >= 10000 {
                c = awful_bad;
            }
            // @!stat if tracing_pages>0 then @<Display the page break cost@>;@+tats@;@/
            region_stat! {
                if tracing_pages!($globals) > 0 {
                    Display_the_page_break_cost!($globals);
                }
            }
            // if c<=least_page_cost then
            if c <= $globals.least_page_cost {
                /// nodes being examined
                let mut r: pointer;
                // begin best_page_break:=p; best_size:=page_goal;
                $globals.best_page_break = $p;
                $globals.best_size = page_goal!($globals);
                // least_page_cost:=c;
                $globals.least_page_cost = c;
                // r:=link(page_ins_head);
                r = link!($globals, page_ins_head);
                // while r<>page_ins_head do
                while r != page_ins_head {
                    // begin best_ins_ptr(r):=last_ins_ptr(r);
                    best_ins_ptr!($globals, r) = last_ins_ptr!($globals, r);
                    // r:=link(r);
                    r = link!($globals, r);
                    // end;
                }
                // end;
            }
            // if (c=awful_bad)or(pi<=eject_penalty) then
            if c == awful_bad || $pi <= eject_penalty {
                // begin fire_up(p); {output the current page at the best place}
                /// output the current page at the best place
                fire_up($globals, $p)?;
                // if output_active then return; {user's output routine will act}
                /// user's output routine will act
                if $globals.output_active {
                    return_nojump!();
                }
                // goto done; {the page has been shipped out by default output routine}
                /// the page has been shipped out by default output routine
                goto_forward_label!($lbl_done);
                // end;
            }
            // end
        }
        use crate::pascal::integer;
        use crate::section_0108::inf_bad;
        use crate::section_0157::inf_penalty;
        use crate::section_0157::eject_penalty;
        use crate::section_0162::page_ins_head;
        use crate::section_0833::awful_bad;
        use crate::section_0974::deplorable;
        use crate::section_1012::fire_up;
    }}
}
