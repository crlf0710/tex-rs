//! @ Sometimes the user will generate a huge page because other error messages
//! are being ignored. Such pages are not output to the \.{dvi} file, since they
//! may confuse the printing software.
//
// @<Update the values of |max_h| and |max_v|; but if the page is too large...@>=
// if (height(p)>max_dimen)or@|(depth(p)>max_dimen)or@|
//    (height(p)+depth(p)+v_offset>max_dimen)or@|
//    (width(p)+h_offset>max_dimen) then
//   begin print_err("Huge page cannot be shipped out");
// @.Huge page...@>
//   help2("The page just created is more than 18 feet tall or")@/
//    ("more than 18 feet wide, so I suspect something went wrong.");
//   error;
//   if tracing_output<=0 then
//     begin begin_diagnostic;
//     print_nl("The following box has been deleted:");
// @.The following...deleted@>
//     show_box(p);
//     end_diagnostic(true);
//     end;
//   goto done;
//   end;
// if height(p)+depth(p)+v_offset>max_v then max_v:=height(p)+depth(p)+v_offset;
// if width(p)+h_offset>max_h then max_h:=width(p)+h_offset
//
