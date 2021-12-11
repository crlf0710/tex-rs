//! @* \[7] Arithmetic with scaled dimensions.
//! The principal computations performed by \TeX\ are done entirely in terms of
//! integers less than $2^{31}$ in magnitude; and divisions are done only when both
//! dividend and divisor are nonnegative. Thus, the arithmetic specified in this
//! program can be carried out in exactly the same way on a wide variety of
//! computers, including some small ones. Why? Because the arithmetic
//! calculations need to be spelled out precisely in order to guarantee that
//! \TeX\ will produce identical output on different machines. If some
//! quantities were rounded differently in different implementations, we would
//! find that line breaks and even page breaks might occur in different places.
//! Hence the arithmetic of \TeX\ has been designed with care, and systems that
//! claim to be implementations of \TeX82 should follow precisely the
//! @:TeX82}{\TeX82@>
//! calculations as they appear in the present program.
//!
//! (Actually there are three places where \TeX\ uses |div| with a possibly negative
//! numerator. These are harmless; see |div| in the index. Also if the user
//! sets the \.{\\time} or the \.{\\year} to a negative value, some diagnostic
//! information will involve negative-numerator division. The same remarks
//! apply for |mod| as well as for |div|.)
//!
