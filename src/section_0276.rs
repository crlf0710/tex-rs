//! @ To save a value of |eqtb[p]| that was established at level |l|, we
//! can use the following subroutine.
//!
//! @p procedure eq_save(@!p:pointer;@!l:quarterword); {saves |eqtb[p]|}
//! begin check_full_save_stack;
//! if l=level_zero then save_type(save_ptr):=restore_zero
//! else  begin save_stack[save_ptr]:=eqtb[p]; incr(save_ptr);
//!   save_type(save_ptr):=restore_old_value;
//!   end;
//! save_level(save_ptr):=l; save_index(save_ptr):=p; incr(save_ptr);
//! end;
//!
