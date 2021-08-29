//! @ Versions of \TeX\ intended for small computers might well choose to omit
//! the ideas in the next few parts of this program, since it is not really
//! necessary to optimize the \.{DVI} code by making use of the |w0|, |x0|,
//! |y0|, and |z0| commands. Furthermore, the algorithm that we are about to
//! describe does not pretend to give an optimum reduction in the length
//! of the \.{DVI} code; after all, speed is more important than compactness.
//! But the method is surprisingly effective, and it takes comparatively little
//! time.
//!
//! We can best understand the basic idea by first considering a simpler problem
//! that has the same essential characteristics. Given a sequence of digits,
//! say $3\,1\,4\,1\,5\,9\,2\,6\,5\,3\,5\,8\,9$, we want to assign subscripts
//! $d$, $y$, or $z$ to each digit so as to maximize the number of ``$y$-hits''
//! and ``$z$-hits''; a $y$-hit is an instance of two appearances of the same
//! digit with the subscript $y$, where no $y$'s intervene between the two
//! appearances, and a $z$-hit is defined similarly. For example, the sequence
//! above could be decorated with subscripts as follows:
//! $$3_z\,1_y\,4_d\,1_y\,5_y\,9_d\,2_d\,6_d\,5_y\,3_z\,5_y\,8_d\,9_d.$$
//! There are three $y$-hits ($1_y\ldots1_y$ and $5_y\ldots5_y\ldots5_y$) and
//! one $z$-hit ($3_z\ldots3_z$); there are no $d$-hits, since the two appearances
//! of $9_d$ have $d$'s between them, but we don't count $d$-hits so it doesn't
//! matter how many there are. These subscripts are analogous to the \.{DVI}
//! commands called \\{down}, $y$, and $z$, and the digits are analogous to
//! different amounts of vertical motion; a $y$-hit or $z$-hit corresponds to
//! the opportunity to use the one-byte commands |y0| or |z0| in a \.{DVI} file.
//!
//! \TeX's method of assigning subscripts works like this: Append a new digit,
//! say $\delta$, to the right of the sequence. Now look back through the
//! sequence until one of the following things happens: (a)~You see
//! $\delta_y$ or $\delta_z$, and this was the first time you encountered a
//! $y$ or $z$ subscript, respectively.  Then assign $y$ or $z$ to the new
//! $\delta$; you have scored a hit. (b)~You see $\delta_d$, and no $y$
//! subscripts have been encountered so far during this search.  Then change
//! the previous $\delta_d$ to $\delta_y$ (this corresponds to changing a
//! command in the output buffer), and assign $y$ to the new $\delta$; it's
//! another hit.  (c)~You see $\delta_d$, and a $y$ subscript has been seen
//! but not a $z$.  Change the previous $\delta_d$ to $\delta_z$ and assign
//! $z$ to the new $\delta$. (d)~You encounter both $y$ and $z$ subscripts
//! before encountering a suitable $\delta$, or you scan all the way to the
//! front of the sequence. Assign $d$ to the new $\delta$; this assignment may
//! be changed later.
//!
//! The subscripts $3_z\,1_y\,4_d\ldots\,$ in the example above were, in fact,
//! produced by this procedure, as the reader can verify. (Go ahead and try it.)
//!
