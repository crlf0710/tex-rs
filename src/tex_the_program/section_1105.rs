//! @ When |delete_last| is called, |cur_chr| is the |type| of node that
//! will be deleted, if present.
//!
//! @<Declare action...@>=
//! procedure delete_last;
//! label exit;
//! var @!p,@!q:pointer; {run through the current list}
//! @!m:quarterword; {the length of a replacement list}
//! begin if (mode=vmode)and(tail=head) then
//!   @<Apologize for inability to do the operation now,
//!     unless \.{\\unskip} follows non-glue@>
//! else  begin if not is_char_node(tail) then if type(tail)=cur_chr then
//!     begin q:=head;
//!     repeat p:=q;
//!     if not is_char_node(q) then if type(q)=disc_node then
//!       begin for m:=1 to replace_count(q) do p:=link(p);
//!       if p=tail then return;
//!       end;
//!     q:=link(p);
//!     until q=tail;
//!     link(p):=null; flush_node_list(tail); tail:=p;
//!     end;
//!   end;
//! exit:end;
//!
