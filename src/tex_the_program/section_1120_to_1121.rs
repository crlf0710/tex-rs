//! @ @<Attach list |p| to the current...@>=
//! begin if (n>0)and(abs(mode)=mmode) then
//!   begin print_err("Illegal math "); print_esc("discretionary");
//! @.Illegal math \\disc...@>
//!   help2("Sorry: The third part of a discretionary break must be")@/
//!   ("empty, in math formulas. I had to delete your third part.");
//!   flush_node_list(p); n:=0; error;
//!   end
//! else link(tail):=p;
//! if n<=max_quarterword then replace_count(tail):=n
//! else  begin print_err("Discretionary list is too long");
//! @.Discretionary list is too long@>
//!   help2("Wow---I never thought anybody would tweak me here.")@/
//!   ("You can't seriously need such a huge discretionary list?");
//!   error;
//!   end;
//! if n>0 then tail:=q;
//! decr(save_ptr); return;
//! end
//!
//! @ During this loop, |p=link(q)| and there are |n| items preceding |p|.
//!
//! @<Prune the current list, if necessary...@>=
//! q:=head; p:=link(q); n:=0;
//! while p<>null do
//!   begin if not is_char_node(p) then if type(p)>rule_node then
//!     if type(p)<>kern_node then if type(p)<>ligature_node then
//!       begin print_err("Improper discretionary list");
//! @.Improper discretionary list@>
//!       help1("Discretionary lists must contain only boxes and kerns.");@/
//!       error;
//!       begin_diagnostic;
//!       print_nl("The following discretionary sublist has been deleted:");
//! @.The following...deleted@>
//!       show_box(p);
//!       end_diagnostic(true);
//!       flush_node_list(p); link(q):=null; goto done;
//!       end;
//!   q:=p; p:=link(q); incr(n);
//!   end;
//! done:
//!
