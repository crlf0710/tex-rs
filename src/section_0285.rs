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
