//! @ Subroutine |save_for_after| puts a token on the stack for save-keeping.
//!
//! @p procedure save_for_after(@!t:halfword);
//! begin if cur_level>level_one then
//!   begin check_full_save_stack;
//!   save_type(save_ptr):=insert_token; save_level(save_ptr):=level_zero;
//!   save_index(save_ptr):=t; incr(save_ptr);
//!   end;
//! end;
//!
//! @ The |unsave| routine goes the other way, taking items off of |save_stack|.
//! This routine takes care of restoration when a level ends; everything
//! belonging to the topmost group is cleared off of the save stack.
//!
//! @p@t\4@>@<Declare the procedure called |restore_trace|@>@;@/
//! procedure@?back_input; forward; @t\2@>
//! procedure unsave; {pops the top level off the save stack}
//! label done;
//! var p:pointer; {position to be restored}
//! @!l:quarterword; {saved level, if in fullword regions of |eqtb|}
//! @!t:halfword; {saved value of |cur_tok|}
//! begin if cur_level>level_one then
//!   begin decr(cur_level);
//!   @<Clear off top level from |save_stack|@>;
//!   end
//! else confusion("curlevel"); {|unsave| is not used when |cur_group=bottom_level|}
//! @:this can't happen curlevel}{\quad curlevel@>
//! end;
//!
//! @ @<Clear off...@>=
//! loop@+begin decr(save_ptr);
//!   if save_type(save_ptr)=level_boundary then goto done;
//!   p:=save_index(save_ptr);
//!   if save_type(save_ptr)=insert_token then
//!     @<Insert token |p| into \TeX's input@>
//!   else  begin if save_type(save_ptr)=restore_old_value then
//!       begin l:=save_level(save_ptr); decr(save_ptr);
//!       end
//!     else save_stack[save_ptr]:=eqtb[undefined_control_sequence];
//!     @<Store \(s)|save_stack[save_ptr]| in |eqtb[p]|, unless
//!       |eqtb[p]| holds a global value@>;
//!     end;
//!   end;
//! done: cur_group:=save_level(save_ptr); cur_boundary:=save_index(save_ptr)
//!
//! @ A global definition, which sets the level to |level_one|,
//! @^global definitions@>
//! will not be undone by |unsave|. If at least one global definition of
//! |eqtb[p]| has been carried out within the group that just ended, the
//! last such definition will therefore survive.
//!
//! @<Store \(s)|save...@>=
//! if p<int_base then
//!   if eq_level(p)=level_one then
//!     begin eq_destroy(save_stack[save_ptr]); {destroy the saved value}
//!     @!stat if tracing_restores>0 then restore_trace(p,"retaining");@+tats@;@/
//!     end
//!   else  begin eq_destroy(eqtb[p]); {destroy the current value}
//!     eqtb[p]:=save_stack[save_ptr]; {restore the saved value}
//!     @!stat if tracing_restores>0 then restore_trace(p,"restoring");@+tats@;@/
//!     end
//! else if xeq_level[p]<>level_one then
//!   begin eqtb[p]:=save_stack[save_ptr]; xeq_level[p]:=l;
//!   @!stat if tracing_restores>0 then restore_trace(p,"restoring");@+tats@;@/
//!   end
//! else  begin
//!   @!stat if tracing_restores>0 then restore_trace(p,"retaining");@+tats@;@/
//!   end
//!
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
//! @ Most of the parameters kept in |eqtb| can be changed freely, but there's
//! an exception:  The magnification should not be used with two different
//! values during any \TeX\ job, since a single magnification is applied to an
//! entire run. The global variable |mag_set| is set to the current magnification
//! whenever it becomes necessary to ``freeze'' it at a particular value.
//!
//! @<Glob...@>=
//! @!mag_set:integer; {if nonzero, this magnification should be used henceforth}
//!
//! @ @<Set init...@>=
//! mag_set:=0;
//!
//! @ The |prepare_mag| subroutine is called whenever \TeX\ wants to use |mag|
//! for magnification.
//!
//! @p procedure prepare_mag;
//! begin if (mag_set>0)and(mag<>mag_set) then
//!   begin print_err("Incompatible magnification ("); print_int(mag);
//! @.Incompatible magnification@>
//!   print(");"); print_nl(" the previous value will be retained");
//!   help2("I can handle only one magnification ratio per job. So I've")@/
//!   ("reverted to the magnification you used earlier on this run.");@/
//!   int_error(mag_set);
//!   geq_word_define(int_base+mag_code,mag_set); {|mag:=mag_set|}
//!   end;
//! if (mag<=0)or(mag>32768) then
//!   begin print_err("Illegal magnification has been changed to 1000");@/
//! @.Illegal magnification...@>
//!   help1("The magnification ratio must be between 1 and 32768.");
//!   int_error(mag); geq_word_define(int_base+mag_code,1000);
//!   end;
//! mag_set:=mag;
//! end;
//!
