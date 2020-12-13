//! @ The following routine is used to implement `\.{\\fontdimen} |n| |f|'.
//! The boolean parameter |writing| is set |true| if the calling program
//! intends to change the parameter value.
//!
//! @<Declare procedures that scan font-related stuff@>=
//! procedure find_font_dimen(@!writing:boolean);
//!   {sets |cur_val| to |font_info| location}
//! var f:internal_font_number;
//! @!n:integer; {the parameter number}
//! begin scan_int; n:=cur_val; scan_font_ident; f:=cur_val;
//! if n<=0 then cur_val:=fmem_ptr
//! else  begin if writing and(n<=space_shrink_code)and@|
//!     (n>=space_code)and(font_glue[f]<>null) then
//!     begin delete_glue_ref(font_glue[f]);
//!     font_glue[f]:=null;
//!     end;
//!   if n>font_params[f] then
//!     if f<font_ptr then cur_val:=fmem_ptr
//!     else @<Increase the number of parameters in the last font@>
//!   else cur_val:=n+param_base[f];
//!   end;
//! @<Issue an error message if |cur_val=fmem_ptr|@>;
//! end;
//!
//! @ @<Issue an error message if |cur_val=fmem_ptr|@>=
//! if cur_val=fmem_ptr then
//!   begin print_err("Font "); print_esc(font_id_text(f));
//!   print(" has only "); print_int(font_params[f]);
//!   print(" fontdimen parameters");
//! @.Font x has only...@>
//!   help2("To increase the number of font parameters, you must")@/
//!     ("use \fontdimen immediately after the \font is loaded.");
//!   error;
//!   end
//!
//! @ @<Increase the number of parameters...@>=
//! begin repeat if fmem_ptr=font_mem_size then
//!   overflow("font memory",font_mem_size);
//! @:TeX capacity exceeded font memory}{\quad font memory@>
//! font_info[fmem_ptr].sc:=0; incr(fmem_ptr); incr(font_params[f]);
//! until n=font_params[f];
//! cur_val:=fmem_ptr-1; {this equals |param_base[f]+font_params[f]|}
//! end
//!
