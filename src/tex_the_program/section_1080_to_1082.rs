//! @ Note that the condition |not is_char_node(tail)| implies that |head<>tail|,
//! since |head| is a one-word node.
//!
//! @<If the current list ends with a box node, delete it...@>=
//! begin cur_box:=null;
//! if abs(mode)=mmode then
//!   begin you_cant; help1("Sorry; this \lastbox will be void."); error;
//!   end
//! else if (mode=vmode)and(head=tail) then
//!   begin you_cant;
//!   help2("Sorry...I usually can't take things from the current page.")@/
//!     ("This \lastbox will therefore be void."); error;
//!   end
//! else  begin if not is_char_node(tail) then
//!     if (type(tail)=hlist_node)or(type(tail)=vlist_node) then
//!       @<Remove the last box, unless it's part of a discretionary@>;
//!   end;
//! end
//!
//! @ @<Remove the last box...@>=
//! begin q:=head;
//! repeat p:=q;
//! if not is_char_node(q) then if type(q)=disc_node then
//!   begin for m:=1 to replace_count(q) do p:=link(p);
//!   if p=tail then goto done;
//!   end;
//! q:=link(p);
//! until q=tail;
//! cur_box:=tail; shift_amount(cur_box):=0;
//! tail:=p; link(p):=null;
//! done:end
//!
//! @ Here we deal with things like `\.{\\vsplit 13 to 100pt}'.
//!
//! @<Split off part of a vertical box, make |cur_box| point to it@>=
//! begin scan_eight_bit_int; n:=cur_val;
//! if not scan_keyword("to") then
//! @.to@>
//!   begin print_err("Missing `to' inserted");
//! @.Missing `to' inserted@>
//!   help2("I'm working on `\vsplit<box number> to <dimen>';")@/
//!   ("will look for the <dimen> next."); error;
//!   end;
//! scan_normal_dimen;
//! cur_box:=vsplit(n,cur_val);
//! end
//!
