//! @ Three labels must be declared in the main program, so we give them
//! symbolic names.
//!
//! @d start_of_TEX=1 {go here when \TeX's variables are initialized}
//! @d end_of_TEX=9998 {go here to close files and terminate gracefully}
//! @d final_end=9999 {this label marks the ending of the program}
//!
//! @<Labels in the out...@>=
//! start_of_TEX@t\hskip-2pt@>, end_of_TEX@t\hskip-2pt@>,@,final_end;
//!   {key control points}
//!
//! @ Some of the code below is intended to be used only when diagnosing the
//! strange behavior that sometimes occurs when \TeX\ is being installed or
//! when system wizards are fooling around with \TeX\ without quite knowing
//! what they are doing. Such code will not normally be compiled; it is
//! delimited by the codewords `$|debug|\ldots|gubed|$', with apologies
//! to people who wish to preserve the purity of English.
//!
//! Similarly, there is some conditional code delimited by
//! `$|stat|\ldots|tats|$' that is intended for use when statistics are to be
//! kept about \TeX's memory usage.  The |stat| $\ldots$ |tats| code also
//! implements diagnostic information for \.{\\tracingparagraphs} and
//! \.{\\tracingpages}.
//! @^debugging@>
//!
//! @d debug==@{ {change this to `$\\{debug}\equiv\null$' when debugging}
//! @d gubed==@t@>@} {change this to `$\\{gubed}\equiv\null$' when debugging}
//! @f debug==begin
//! @f gubed==end
//! @#
//! @d stat==@{ {change this to `$\\{stat}\equiv\null$' when gathering
//!   usage statistics}
//! @d tats==@t@>@} {change this to `$\\{tats}\equiv\null$' when gathering
//!   usage statistics}
//! @f stat==begin
//! @f tats==end
//!
//! @ This program has two important variations: (1) There is a long and slow
//! version called \.{INITEX}, which does the extra calculations needed to
//! @.INITEX@>
//! initialize \TeX's internal tables; and (2)~there is a shorter and faster
//! production version, which cuts the initialization to a bare minimum.
//! Parts of the program that are needed in (1) but not in (2) are delimited by
//! the codewords `$|init|\ldots|tini|$'.
//!
//! @d init== {change this to `$\\{init}\equiv\.{@@\{}$' in the production version}
//! @d tini== {change this to `$\\{tini}\equiv\.{@@\}}$' in the production version}
//! @f init==begin
//! @f tini==end
//!
//! @<Initialize whatever...@>=
//! @<Set initial values of key variables@>@/
//! @!init @<Initialize table entries (done by \.{INITEX} only)@>@;@+tini
//!
//! @ If the first character of a \PASCAL\ comment is a dollar sign,
//! \ph\ treats the comment as a list of ``compiler directives'' that will
//! affect the translation of this program into machine language.  The
//! directives shown below specify full checking and inclusion of the \PASCAL\
//! debugger when \TeX\ is being debugged, but they cause range checking and other
//! redundant code to be eliminated when the production system is being generated.
//! Arithmetic overflow will be detected in all cases.
//! @:PASCAL H}{\ph@>
//! @^system dependencies@>
//! @^overflow in arithmetic@>
//!
//! @<Compiler directives@>=
//! @{@&$C-,A+,D-@} {no range check, catch arithmetic overflow, no debug overhead}
//! @!debug @{@&$C+,D+@}@+ gubed {but turn everything on when debugging}
//!
//! @ This \TeX\ implementation conforms to the rules of the {\sl Pascal User
//! @:PASCAL}{\PASCAL@>
//! @^system dependencies@>
//! Manual} published by Jensen and Wirth in 1975, except where system-dependent
//! @^Wirth, Niklaus@>
//! @^Jensen, Kathleen@>
//! code is necessary to make a useful system program, and except in another
//! respect where such conformity would unnecessarily obscure the meaning
//! and clutter up the code: We assume that |case| statements may include a
//! default case that applies if no matching label is found. Thus, we shall use
//! constructions like
//! $$\vbox{\halign{\ignorespaces#\hfil\cr
//! |case x of|\cr
//! 1: $\langle\,$code for $x=1\,\rangle$;\cr
//! 3: $\langle\,$code for $x=3\,\rangle$;\cr
//! |othercases| $\langle\,$code for |x<>1| and |x<>3|$\,\rangle$\cr
//! |endcases|\cr}}$$
//! since most \PASCAL\ compilers have plugged this hole in the language by
//! incorporating some sort of default mechanism. For example, the \ph\
//! compiler allows `|others|:' as a default label, and other \PASCAL s allow
//! syntaxes like `\&{else}' or `\&{otherwise}' or `\\{otherwise}:', etc. The
//! definitions of |othercases| and |endcases| should be changed to agree with
//! local conventions.  Note that no semicolon appears before |endcases| in
//! this program, so the definition of |endcases| should include a semicolon
//! if the compiler wants one. (Of course, if no default mechanism is
//! available, the |case| statements of \TeX\ will have to be laboriously
//! extended by listing all remaining cases. People who are stuck with such
//! \PASCAL s have, in fact, done this, successfully but not happily!)
//! @:PASCAL H}{\ph@>
//!
//! @d othercases == others: {default for cases not listed explicitly}
//! @d endcases == @+end {follows the default case in an extended |case| statement}
//! @f othercases == else
//! @f endcases == end
//!
