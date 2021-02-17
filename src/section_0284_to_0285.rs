//! @ @<Declare the procedure called |restore_trace|@>=
//! @!stat procedure restore_trace(@!p:pointer;@!s:str_number);
//!   {|eqtb[p]| has just been restored or retained}
//! begin begin_diagnostic; print_char("{"); print(s); print_char(" ");
//! show_eqtb(p); print_char("}");
//! end_diagnostic(false);
//! end;
//! tats
//!
//! @ When looking for possible pointers to a memory location, it is helpful
//! to look for references from |eqtb| that might be waiting on the
//! save stack. Of course, we might find spurious pointers too; but this
//! routine is merely an aid when debugging, and at such times we are
//! grateful for any scraps of information, even if they prove to be irrelevant.
//! @^dirty \PASCAL@>
//!
//! @<Search |save_stack| for equivalents that point to |p|@>=
//! if save_ptr>0 then for q:=0 to save_ptr-1 do
//!   begin if equiv_field(save_stack[q])=p then
//!     begin print_nl("SAVE("); print_int(q); print_char(")");
//!     end;
//!   end
//!
