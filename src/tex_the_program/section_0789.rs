//! @ The scanner sets |align_state| to zero when the \<u_j> template ends. When
//! a subsequent \.{\\cr} or \.{\\span} or tab mark occurs with |align_state=0|,
//! the scanner activates the following code, which fires up the \<v_j> template.
//! We need to remember the |cur_chr|, which is either |cr_cr_code|, |cr_code|,
//! |span_code|, or a character code, depending on how the column text has ended.
//!
//! This part of the program had better not be activated when the preamble
//! to another alignment is being scanned, or when no alignment preamble is active.
//
// @<Insert the \(v)\<v_j>...@>=
macro_rules! Insert_the_v_j_template_and_goto_restart {
    ($globals:expr, $lbl_restart:lifetime) => {{
        trace_span!("Insert the v_j...");
        // begin if (scanner_status=aligning) or (cur_align=null) then
        //   fatal_error("(interwoven alignment preambles are not allowed)");
        // @.interwoven alignment preambles...@>
        // cur_cmd:=extra_info(cur_align); extra_info(cur_align):=cur_chr;
        // if cur_cmd=omit then begin_token_list(omit_template,v_template)
        if $globals.cur_cmd == omit {
            begin_token_list($globals, omit_template, v_template);
        }
        // else begin_token_list(v_part(cur_align),v_template);
        else {
            begin_token_list($globals, v_part!($globals, $globals.cur_align) as _,v_template);
        }
        // align_state:=1000000; goto restart;
        $globals.align_state = 1000000;
        goto_backward_label!($lbl_restart);
        // end
        use crate::section_0307::v_template;
        use crate::section_0162::omit_template;
        use crate::section_0208::omit;
        use crate::section_0323::begin_token_list;
    }}
}
