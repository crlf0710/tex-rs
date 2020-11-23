//! ` `

// @<Put help message on the transcript file@>=
macro_rules! Put_help_message_on_the_transcript_file {
    ($globals:expr) => {{
        todo!("put help message");
        // if interaction>batch_mode then decr(selector); {avoid terminal output}
        // if use_err_help then
        //   begin print_ln; give_err_help;
        //   end
        // else while help_ptr>0 do
        //   begin decr(help_ptr); print_nl(help_line[help_ptr]);
        //   end;
        // print_ln;
        // if interaction>batch_mode then incr(selector); {re-enable terminal output}
        // print_ln
    }}
}
