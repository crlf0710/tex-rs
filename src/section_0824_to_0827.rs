//! @ Let's state the principles of the delta nodes more precisely and concisely,
//! so that the following programs will be less obscure. For each legal
//! breakpoint~|p| in the paragraph, we define two quantities $\alpha(p)$ and
//! $\beta(p)$ such that the length of material in a line from breakpoint~|p|
//! to breakpoint~|q| is $\gamma+\beta(q)-\alpha(p)$, for some fixed $\gamma$.
//! Intuitively, $\alpha(p)$ and $\beta(q)$ are the total length of material from
//! the beginning of the paragraph to a point ``after'' a break at |p| and to a
//! point ``before'' a break at |q|; and $\gamma$ is the width of an empty line,
//! namely the length contributed by \.{\\leftskip} and \.{\\rightskip}.
//!
//! Suppose, for example, that the paragraph consists entirely of alternating
//! boxes and glue skips; let the boxes have widths $x_1\ldots x_n$ and
//! let the skips have widths $y_1\ldots y_n$, so that the paragraph can be
//! represented by $x_1y_1\ldots x_ny_n$. Let $p_i$ be the legal breakpoint
//! at $y_i$; then $\alpha(p_i)=x_1+y_1+\cdots+x_i+y_i$, and $\beta(p_i)=
//! x_1+y_1+\cdots+x_i$. To check this, note that the length of material from
//! $p_2$ to $p_5$, say, is $\gamma+x_3+y_3+x_4+y_4+x_5=\gamma+\beta(p_5)
//! -\alpha(p_2)$.
//!
//! The quantities $\alpha$, $\beta$, $\gamma$ involve glue stretchability and
//! shrinkability as well as a natural width. If we were to compute $\alpha(p)$
//! and $\beta(p)$ for each |p|, we would need multiple precision arithmetic, and
//! the multiprecise numbers would have to be kept in the active nodes.
//! \TeX\ avoids this problem by working entirely with relative differences
//! or ``deltas.'' Suppose, for example, that the active list contains
//! $a_1\,\delta_1\,a_2\,\delta_2\,a_3$, where the |a|'s are active breakpoints
//! and the $\delta$'s are delta nodes. Then $\delta_1=\alpha(a_1)-\alpha(a_2)$
//! and $\delta_2=\alpha(a_2)-\alpha(a_3)$. If the line breaking algorithm is
//! currently positioned at some other breakpoint |p|, the |active_width| array
//! contains the value $\gamma+\beta(p)-\alpha(a_1)$. If we are scanning through
//! the list of active nodes and considering a tentative line that runs from
//! $a_2$ to~|p|, say, the |cur_active_width| array will contain the value
//! $\gamma+\beta(p)-\alpha(a_2)$. Thus, when we move from $a_2$ to $a_3$,
//! we want to add $\alpha(a_2)-\alpha(a_3)$ to |cur_active_width|; and this
//! is just $\delta_2$, which appears in the active list between $a_2$ and
//! $a_3$. The |background| array contains $\gamma$. The |break_width| array
//! will be used to calculate values of new delta nodes when the active
//! list is being updated.
//!
//! @ Glue nodes in a horizontal list that is being paragraphed are not supposed to
//! include ``infinite'' shrinkability; that is why the algorithm maintains
//! four registers for stretching but only one for shrinking. If the user tries to
//! introduce infinite shrinkability, the shrinkability will be reset to finite
//! and an error message will be issued. A boolean variable |no_shrink_error_yet|
//! prevents this error message from appearing more than once per paragraph.
//!
//! @d check_shrinkage(#)==if (shrink_order(#)<>normal)and(shrink(#)<>0) then
//!   begin #:=finite_shrink(#);
//!   end
//!
//! @<Glob...@>=
//! @!no_shrink_error_yet:boolean; {have we complained about infinite shrinkage?}
//!
//! @ @<Declare subprocedures for |line_break|@>=
//! function finite_shrink(@!p:pointer):pointer; {recovers from infinite shrinkage}
//! var q:pointer; {new glue specification}
//! begin if no_shrink_error_yet then
//!   begin no_shrink_error_yet:=false;
//!   print_err("Infinite glue shrinkage found in a paragraph");
//! @.Infinite glue shrinkage...@>
//!   help5("The paragraph just ended includes some glue that has")@/
//!   ("infinite shrinkability, e.g., `\hskip 0pt minus 1fil'.")@/
//!   ("Such glue doesn't belong there---it allows a paragraph")@/
//!   ("of any length to fit on one line. But it's safe to proceed,")@/
//!   ("since the offensive shrinkability has been made finite.");
//!   error;
//!   end;
//! q:=new_spec(p); shrink_order(q):=normal;
//! delete_glue_ref(p); finite_shrink:=q;
//! end;
//!
//! @ @<Get ready to start...@>=
//! no_shrink_error_yet:=true;@/
//! check_shrinkage(left_skip); check_shrinkage(right_skip);@/
//! q:=left_skip; r:=right_skip; background[1]:=width(q)+width(r);@/
//! background[2]:=0; background[3]:=0; background[4]:=0; background[5]:=0;@/
//! background[2+stretch_order(q)]:=stretch(q);@/
//! background[2+stretch_order(r)]:=@|background[2+stretch_order(r)]+stretch(r);@/
//! background[6]:=shrink(q)+shrink(r);
//!
