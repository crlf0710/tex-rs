//! @ Here we delete node |p| from the ring, and let |rover| rove around.
//!
//! @<Allocate entire...@>=
//! begin rover:=rlink(p); t:=llink(p);
//! llink(rover):=t; rlink(t):=rover;
//! goto found;
//! end
//!
