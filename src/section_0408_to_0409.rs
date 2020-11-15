//! @ Here is a procedure that sounds an alarm when mu and non-mu units
//! are being switched.
//!
//! @p procedure mu_error;
//! begin print_err("Incompatible glue units");
//! @.Incompatible glue units@>
//! help1("I'm going to assume that 1mu=1pt when they're mixed.");
//! error;
//! end;
//!
//! @ The next routine `|scan_something_internal|' is used to fetch internal
//! numeric quantities like `\.{\\hsize}', and also to handle the `\.{\\the}'
//! when expanding constructions like `\.{\\the\\toks0}' and
//! `\.{\\the\\baselineskip}'. Soon we will be considering the |scan_int|
//! procedure, which calls |scan_something_internal|; on the other hand,
//! |scan_something_internal| also calls |scan_int|, for constructions like
//! `\.{\\catcode\`\\\$}' or `\.{\\fontdimen} \.3 \.{\\ff}'. So we
//! have to declare |scan_int| as a |forward| procedure. A few other
//! procedures are also declared at this point.
//!
//! @p procedure@?scan_int; forward; {scans an integer value}
//! @t\4\4@>@<Declare procedures that scan restricted classes of integers@>@;
//! @t\4\4@>@<Declare procedures that scan font-related stuff@>
//!
