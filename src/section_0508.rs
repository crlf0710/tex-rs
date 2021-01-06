//! @ Note also that `\.{\\ifx}' decides that macros \.{\\a} and \.{\\b} are
//! different in examples like this:
//! $$\vbox{\halign{\.{#}\hfil&\qquad\.{#}\hfil\cr
//!   {}\\def\\a\{\\c\}&
//!   {}\\def\\c\{\}\cr
//!   {}\\def\\b\{\\d\}&
//!   {}\\def\\d\{\}\cr}}$$
//
// @<Test if two macro texts match@>=
macro_rules! Test_if_two_macro_texts_match {
    ($globals:expr, $b:expr, $n:expr) => {{
        let (mut p, mut q);
        // begin p:=link(cur_chr); q:=link(equiv(n)); {omit reference counts}
        /// omit reference counts
        const _ : () = ();
        p = link!($globals, $globals.cur_chr.get() as pointer);
        q = link!($globals, equiv!($globals, $n));
        // if p=q then b:=true
        if p == q {
            $b = true;
        }
        // else begin while (p<>null)and(q<>null) do
        else {
            while p != null && q != null {
                // if info(p)<>info(q) then p:=null
                if info_tok!($globals, p) != info_tok!($globals, q) {
                    p = null;
                }
                // else  begin p:=link(p); q:=link(q);
                else {
                    p = link!($globals, p);
                    q = link!($globals, q);
                    // end;
                }
            }
            // b:=((p=null)and(q=null));
            $b = p == null && q == null;
            // end;
        }
        // end
        use crate::section_0115::null;
    }}
}
