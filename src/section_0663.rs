//! ` `

// @<Finish issuing a diagnostic message for an overfull or underfull hbox@>=
macro_rules! Finish_issuing_a_diagnostic_message_for_an_overfull_or_underfull_hbox {
    ($globals:expr) => {{
        // if output_active then print(") has occurred while \output is active")
        // else  begin if pack_begin_line<>0 then
        //     begin if pack_begin_line>0 then print(") in paragraph at lines ")
        //     else print(") in alignment at lines ");
        //     print_int(abs(pack_begin_line));
        //     print("--");
        //     end
        //   else print(") detected at line ");
        //   print_int(line);
        //   end;
        // print_ln;@/
        // font_in_short_display:=null_font; short_display(list_ptr(r)); print_ln;@/
        // begin_diagnostic; show_box(r); end_diagnostic(true)
        todo!("issue message");
    }}
}