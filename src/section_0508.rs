//! @ Note also that `\.{\\ifx}' decides that macros \.{\\a} and \.{\\b} are
//! different in examples like this:
//! $$\vbox{\halign{\.{#}\hfil&\qquad\.{#}\hfil\cr
//!   {}\\def\\a\{\\c\}&
//!   {}\\def\\c\{\}\cr
//!   {}\\def\\b\{\\d\}&
//!   {}\\def\\d\{\}\cr}}$$
//!
//! @<Test if two macro texts match@>=
//! begin p:=link(cur_chr); q:=link(equiv(n)); {omit reference counts}
//! if p=q then b:=true
//! else begin while (p<>null)and(q<>null) do
//!     if info(p)<>info(q) then p:=null
//!     else  begin p:=link(p); q:=link(q);
//!       end;
//!   b:=((p=null)and(q=null));
//!   end;
//! end
//!
