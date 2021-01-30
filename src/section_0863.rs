//! @ The `\ignorespaces|loop|\unskip' in the following code is performed at most
//! thrice per call of |line_break|, since it is actually a pass over the
//! entire paragraph.
//
// @<Find optimal breakpoints@>=
macro_rules! Find_optimal_breakpoints {
    ($globals:expr) => {{
        // threshold:=pretolerance;
        $globals.threshold = pretolerance!($globals);
        // if threshold>=0 then
        if $globals.threshold >= 0 {
            // begin @!stat if tracing_paragraphs>0 then
            region_stat! {
                if tracing_paragraphs!($globals) > 0 {
                    // begin begin_diagnostic; print_nl("@@firstpass");@+end;@;@+tats@;@/
                    begin_diagnostic($globals);
                    print_nl($globals, strpool_str!("@@firstpass"));
                }
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
            region_stat! {
                if tracing_paragraphs!($globals) > 0 {
                    begin_diagnostic($globals);
                }
            }
            // end;
        }
        // loop@+  begin if threshold>inf_bad then threshold:=inf_bad;
        loop {
            if $globals.threshold as integer > inf_bad as integer {
                $globals.threshold = inf_bad as _;
            }
            todo!();
            // if second_pass then @<Initialize for hyphenating a paragraph@>;
            // @<Create an active breakpoint representing the beginning of the paragraph@>;
            // cur_p:=link(temp_head); auto_breaking:=true;@/
            // prev_p:=cur_p; {glue at beginning is not a legal breakpoint}
            // while (cur_p<>null)and(link(active)<>last_active) do
            //   @<Call |try_break| if |cur_p| is a legal breakpoint;
            //   on the second pass, also try to hyphenate the next
            //   word, if |cur_p| is a glue node;
            //   then advance |cur_p| to the next node of the paragraph
            //   that could possibly be a legal breakpoint@>;
            // if cur_p=null then
            //   @<Try the final line break at the end of the paragraph,
            //   and |goto done| if the desired breakpoints have been found@>;
            // @<Clean up the memory by removing the break nodes@>;
            // if not second_pass then
            //   begin@!stat if tracing_paragraphs>0 then print_nl("@@secondpass");@;@+tats@/
            //   threshold:=tolerance; second_pass:=true; final_pass:=(emergency_stretch<=0);
            //   end {if at first you don't succeed, \dots}
            // else begin @!stat if tracing_paragraphs>0 then
            //     print_nl("@@emergencypass");@;@+tats@/
            //   background[2]:=background[2]+emergency_stretch; final_pass:=true;
            //   end;
            // end;
        }
        // done: @!stat if tracing_paragraphs>0 then
        region_stat! {
            if tracing_paragraphs!($globals) > 0 {
                // begin end_diagnostic(true); normalize_selector;
                end_diagnostic($globals, true);
                normalize_selector($globals);
                // end;@+tats@/
            }
        }

        use crate::section_0101::scaled;
        use crate::section_0108::inf_bad;
    }}
}