//! ` `
// @<Issue an error message if |cur_val=fmem_ptr|@>=
macro_rules! Issue_an_error_message_if_cur_val_eq_fmem_ptr {
    ($globals:expr) => {{
        // if cur_val=fmem_ptr then
        if $globals.cur_val == $globals.fmem_ptr.get() as integer {
            todo!("issue error message");
            //   begin print_err("Font "); print_esc(font_id_text(f));
            //   print(" has only "); print_int(font_params[f]);
            //   print(" fontdimen parameters");
            // @.Font x has only...@>
            //   help2("To increase the number of font parameters, you must")@/
            //     ("use \fontdimen immediately after the \font is loaded.");
            //   error;
            //   end
        }
    }}
}
