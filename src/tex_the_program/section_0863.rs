//! @ The `\ignorespaces|loop|\unskip' in the following code is performed at most
//! thrice per call of |line_break|, since it is actually a pass over the
//! entire paragraph.
//
// @<Find optimal breakpoints@>=
pub(crate) macro Find_optimal_breakpoints
    ($globals:expr) {#[allow(unused_assignments)]{
        crate::trace_span!("Find optimal breakpoints");
        // threshold:=pretolerance;
        $globals.threshold = pretolerance!($globals);
        // if threshold>=0 then
        if $globals.threshold >= 0 {
            // begin @!stat if tracing_paragraphs>0 then
            crate::region_stat! {
                if tracing_paragraphs!($globals) > 0 {
                    // begin begin_diagnostic; print_nl("@@firstpass");@+end;@;@+tats@;@/
                    begin_diagnostic($globals);
                    print_nl($globals, crate::strpool_str!("@firstpass"));
                }
                use crate::section_0245::begin_diagnostic;
                use crate::section_0062::print_nl;
            }
            // second_pass:=false; final_pass:=false;
            $globals.second_pass = false;
            $globals.final_pass = false;
            // end
        }
        // else  begin threshold:=tolerance; second_pass:=true;
        else {
            $globals.threshold = tolerance!($globals);
            $globals.second_pass = true;
            // final_pass:=(emergency_stretch<=0);
            $globals.final_pass = emergency_stretch!($globals) <= scaled::zero();
            // @!stat if tracing_paragraphs>0 then begin_diagnostic;@+tats@;
            crate::region_stat! {
                if tracing_paragraphs!($globals) > 0 {
                    begin_diagnostic($globals);
                }
                use crate::section_0245::begin_diagnostic;
            }
            // end;
        }
        crate::region_forward_label! {
        |'done|
        {
        // loop@+  begin if threshold>inf_bad then threshold:=inf_bad;
        loop {
            /// is node |cur_p| outside a formula?
            let auto_breaking: boolean;
            /// helps to determine when glue nodes are breakpoints
            let mut prev_p: pointer;

            if $globals.threshold as integer > inf_bad as integer {
                $globals.threshold = inf_bad as _;
            }
            // if second_pass then @<Initialize for hyphenating a paragraph@>;
            if $globals.second_pass {
                crate::section_0891::Initialize_for_hyphenating_a_paragraph!($globals);
            }
            // @<Create an active breakpoint representing the beginning of the paragraph@>;
            crate::section_0864::Create_an_active_breakpoint_representing_the_beginning_of_the_paragraph!($globals);
            // cur_p:=link(temp_head); auto_breaking:=true;@/
            $globals.cur_p = link!($globals, temp_head);
            auto_breaking = true;
            // prev_p:=cur_p; {glue at beginning is not a legal breakpoint}
            /// glue at beginning is not a legal breakpoint
            const _ : () = ();
            prev_p = $globals.cur_p;
            // while (cur_p<>null)and(link(active)<>last_active) do
            while $globals.cur_p != null && link!($globals, active) != last_active!() {
                // @<Call |try_break| if |cur_p| is a legal breakpoint;
                // on the second pass, also try to hyphenate the next
                // word, if |cur_p| is a glue node;
                // then advance |cur_p| to the next node of the paragraph
                // that could possibly be a legal breakpoint@>;
                crate::section_0866::Call_try_break_if_cur_p_is_a_legal_breakpoint__on_the_second_pass__also_try_to_hyphenate_the_next_word__if_cur_p_is_a_glue_node__then_advance_cur_p_to_the_next_node_of_the_paragraph_that_could_possibly_be_a_legal_breakpoint!
                    ($globals, prev_p, auto_breaking);
            }
            // if cur_p=null then
            if $globals.cur_p == null {
                // @<Try the final line break at the end of the paragraph,
                // and |goto done| if the desired breakpoints have been found@>;
                crate::section_0873::Try_the_final_line_break_at_the_end_of_the_paragraph__and_goto_done_if_the_desired_breakpoints_have_been_found!
                    ($globals, 'done);
            }
            // @<Clean up the memory by removing the break nodes@>;
            crate::section_0865::Clean_up_the_memory_by_removing_the_break_nodes!($globals);
            // if not second_pass then
            if !$globals.second_pass {
                // begin@!stat if tracing_paragraphs>0 then print_nl("@@secondpass");@;@+tats@/
                crate::region_stat! {
                    if tracing_paragraphs!($globals) > 0 {
                        print_nl($globals, crate::strpool_str!("@secondpass"));
                    }
                    use crate::section_0062::print_nl;
                }
                // threshold:=tolerance; second_pass:=true; final_pass:=(emergency_stretch<=0);
                $globals.threshold = tolerance!($globals);
                $globals.second_pass = true;
                $globals.final_pass = emergency_stretch!($globals) <= scaled::zero();
                // end {if at first you don't succeed, \dots}
                /// if at first you don't succeed, ...
                const _ : () = ();
            }
            // else begin @!stat if tracing_paragraphs>0 then
            else {
                crate::region_stat! {
                    if tracing_paragraphs!($globals) > 0 {
                        // print_nl("@@emergencypass");@;@+tats@/
                        print_nl($globals, crate::strpool_str!("@emergencypass"));
                    }
                    use crate::section_0062::print_nl;
                }
                // background[2]:=background[2]+emergency_stretch; final_pass:=true;
                $globals.background[2] += emergency_stretch!($globals);
                $globals.final_pass = true;
                // end;
            }
            // end;
        }
        }
        // done: @!stat if tracing_paragraphs>0 then
        'done <-
        }
        crate::region_stat! {
            if tracing_paragraphs!($globals) > 0 {
                // begin end_diagnostic(true); normalize_selector;
                end_diagnostic($globals, true);
                normalize_selector($globals);
                // end;@+tats@/
            }
            use crate::section_0092::normalize_selector;
            use crate::section_0245::end_diagnostic;
        }

        use crate::pascal::boolean;
        use crate::pascal::integer;
        use crate::section_0101::scaled;
        use crate::section_0108::inf_bad;
        use crate::section_0115::null;
        use crate::section_0115::pointer;
        use crate::section_0118::link;
        use crate::section_0162::active;
        use crate::section_0162::temp_head;
        use crate::section_0236::tolerance;
        use crate::section_0236::pretolerance;
        use crate::section_0247::emergency_stretch;
        use crate::section_0819::last_active;
    }}
