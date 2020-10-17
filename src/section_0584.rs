//! @ The \.{DVI} format is intended to be both compact and easily interpreted
//! by a machine. Compactness is achieved by making most of the information
//! implicit instead of explicit. When a \.{DVI}-reading program reads the
//! commands for a page, it keeps track of several quantities: (a)~The current
//! font |f| is an integer; this value is changed only
//! by \\{fnt} and \\{fnt\_num} commands. (b)~The current position on the page
//! is given by two numbers called the horizontal and vertical coordinates,
//! |h| and |v|. Both coordinates are zero at the upper left corner of the page;
//! moving to the right corresponds to increasing the horizontal coordinate, and
//! moving down corresponds to increasing the vertical coordinate. Thus, the
//! coordinates are essentially Cartesian, except that vertical directions are
//! flipped; the Cartesian version of |(h,v)| would be |(h,-v)|.  (c)~The
//! current spacing amounts are given by four numbers |w|, |x|, |y|, and |z|,
//! where |w| and~|x| are used for horizontal spacing and where |y| and~|z|
//! are used for vertical spacing. (d)~There is a stack containing
//! |(h,v,w,x,y,z)| values; the \.{DVI} commands |push| and |pop| are used to
//! change the current level of operation. Note that the current font~|f| is
//! not pushed and popped; the stack contains only information about
//! positioning.
//!
//! The values of |h|, |v|, |w|, |x|, |y|, and |z| are signed integers having up
//! to 32 bits, including the sign. Since they represent physical distances,
//! there is a small unit of measurement such that increasing |h| by~1 means
//! moving a certain tiny distance to the right. The actual unit of
//! measurement is variable, as explained below; \TeX\ sets things up so that
//! its \.{DVI} output is in sp units, i.e., scaled points, in agreement with
//! all the |scaled| dimensions in \TeX's data structures.
//!
