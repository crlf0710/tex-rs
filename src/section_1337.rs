//! @ When we begin the following code, \TeX's tables may still contain garbage;
//! the strings might not even be present. Thus we must proceed cautiously to get
//! bootstrapped in.
//!
//! But when we finish this part of the program, \TeX\ is ready to call on the
//! |main_control| routine to do its work.

// @<Get the first line...@>=
macro_rules! Get_the_first_line_of_input_and_prepare_to_start {
    ($globals:expr) => {
        // begin @<Initialize the input routines@>;
        // if (format_ident=0)or(buffer[loc]="&") then
        //   begin if format_ident<>0 then initialize; {erase preloaded format}
        //   if not open_fmt_file then goto final_end;
        //   if not load_fmt_file then
        //     begin w_close(fmt_file); goto final_end;
        //     end;
        //   w_close(fmt_file);
        //   while (loc<limit)and(buffer[loc]=" ") do incr(loc);
        //   end;
        // if end_line_char_inactive then decr(limit)
        // else  buffer[limit]:=end_line_char;
        // fix_date_and_time;@/
        // @<Compute the magic offset@>;
        // @<Initialize the print |selector|...@>;
        // if (loc<limit)and(cat_code(buffer[loc])<>escape) then start_input;
        //   {\.{\\input} assumed}
        // end
    };
}
