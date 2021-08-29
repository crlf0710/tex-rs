//! @ The algorithm essentially determines the best possible way to achieve
//! each feasible combination of position, line, and fitness. Thus, it answers
//! questions like, ``What is the best way to break the opening part of the
//! paragraph so that the fourth line is a tight line ending at such-and-such
//! a place?'' However, the fact that all lines are to be the same length
//! after a certain point makes it possible to regard all sufficiently large
//! line numbers as equivalent, when the looseness parameter is zero, and this
//! makes it possible for the algorithm to save space and time.
//!
//! An ``active node'' and a ``passive node'' are created in |mem| for each
//! feasible breakpoint that needs to be considered. Active nodes are three
//! words long and passive nodes are two words long. We need active nodes only
//! for breakpoints near the place in the paragraph that is currently being
//! examined, so they are recycled within a comparatively short time after
//! they are created.
//!
