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
