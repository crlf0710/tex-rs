//! @ Here is a subroutine that creates a new glue specification from another
//! one that is expressed in `\.{mu}', given the value of the math unit.

// @d mu_mult(#)==nx_plus_y(n,#,xn_over_d(#,f,@'200000))
pub(crate) macro mu_mult($globals:expr, $v:expr, $n:expr, $f:expr) {{
    let y = xn_over_d($globals, $v, $f.inner(), 0o200000);
    nx_plus_y!($globals, $n, $v, y)
}}

// @p function math_glue(@!g:pointer;@!m:scaled):pointer;
pub(crate) fn math_glue(globals: &mut TeXGlobals, g: pointer, m: scaled) -> TeXResult<pointer> {
    // var p:pointer; {the new glue specification}
    /// the new glue specification
    let p;
    // @!n:integer; {integer part of |m|}
    /// integer part of `m`
    let mut n: integer;
    // @!f:scaled; {fraction part of |m|}
    /// fraction part of `m`
    let mut f: scaled;
    // begin n:=x_over_n(m,@'200000); f:=remainder;@/
    n = x_over_n(globals, m, 0o200000).inner();
    f = globals.remainder;
    // if f<0 then
    if f < scaled::zero() {
        // begin decr(n); f:=f+@'200000;
        decr!(n);
        f = scaled::new_from_inner(f.inner() + 0o200000);
        // end;
    }
    // p:=get_node(glue_spec_size);
    p = get_node(globals, glue_spec_size as _)?;
    // width(p):=mu_mult(width(g)); {convert \.{mu} to \.{pt}}
    /// convert `mu` to `pt`
    const _: () = ();
    width!(globals, p) = mu_mult!(globals, width!(globals, g), n, f);
    // stretch_order(p):=stretch_order(g);
    stretch_order!(globals, p) = stretch_order!(globals, g);
    // if stretch_order(p)=normal then stretch(p):=mu_mult(stretch(g))
    if stretch_order!(globals, p) == glue_ord::normal as _ {
        stretch!(globals, p) = mu_mult!(globals, stretch!(globals, g), n, f);
    }
    // else stretch(p):=stretch(g);
    else {
        stretch!(globals, p) = stretch!(globals, g);
    }
    // shrink_order(p):=shrink_order(g);
    shrink_order!(globals, p) = shrink_order!(globals, g);
    // if shrink_order(p)=normal then shrink(p):=mu_mult(shrink(g))
    if shrink_order!(globals, p) == glue_ord::normal as _ {
        shrink!(globals, p) = mu_mult!(globals, shrink!(globals, g), n, f);
    }
    // else shrink(p):=shrink(g);
    else {
        shrink!(globals, p) = shrink!(globals, g);
    }
    // math_glue:=p;
    crate::ok_nojump!(p)
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0105::nx_plus_y;
use crate::section_0106::x_over_n;
use crate::section_0107::xn_over_d;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0135::width;
use crate::section_0150::glue_ord;
use crate::section_0150::glue_spec_size;
use crate::section_0150::shrink;
use crate::section_0150::shrink_order;
use crate::section_0150::stretch;
use crate::section_0150::stretch_order;
