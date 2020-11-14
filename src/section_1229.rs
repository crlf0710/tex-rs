//! @ When a glue register or parameter becomes zero, it will always point to
//! |zero_glue| because of the following procedure. (Exception: The tabskip
//! glue isn't trapped while preambles are being scanned.)
//
// @<Declare subprocedures for |prefixed_command|@>=
// procedure trap_zero_glue;
// begin if (width(cur_val)=0)and(stretch(cur_val)=0)and(shrink(cur_val)=0) then
//   begin add_glue_ref(zero_glue);
//   delete_glue_ref(cur_val); cur_val:=zero_glue;
//   end;
// end;
//
