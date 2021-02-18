//! @ When the |movement| procedure gets to the label |found|, the value of
//! |info(p)| will be either |y_here| or |z_here|. If it is, say, |y_here|,
//! the procedure generates a |y0| command (or a |w0| command), and marks
//! all |info| fields between |q| and |p| so that |y| is not OK in that range.
//!
//! @<Generate a |y0| or |z0| command...@>=
//! info(q):=info(p);
//! if info(q)=y_here then
//!   begin dvi_out(o+y0-down1); {|y0| or |w0|}
//!   while link(q)<>p do
//!     begin q:=link(q);
//!     case info(q) of
//!     yz_OK: info(q):=z_OK;
//!     y_OK: info(q):=d_fixed;
//!     othercases do_nothing
//!     endcases;
//!     end;
//!   end
//! else  begin dvi_out(o+z0-down1); {|z0| or |x0|}
//!   while link(q)<>p do
//!     begin q:=link(q);
//!     case info(q) of
//!     yz_OK: info(q):=y_OK;
//!     z_OK: info(q):=d_fixed;
//!     othercases do_nothing
//!     endcases;
//!     end;
//!   end
//!
