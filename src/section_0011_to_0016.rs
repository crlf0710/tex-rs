//! @ The following parameters can be changed at compile time to extend or
//! reduce \TeX's capacity. They may have different values in \.{INITEX} and
//! in production versions of \TeX.
//! @.INITEX@>
//! @^system dependencies@>
//!
//! @<Constants...@>=
//! @!mem_max=30000; {greatest index in \TeX's internal |mem| array;
//!   must be strictly less than |max_halfword|;
//!   must be equal to |mem_top| in \.{INITEX}, otherwise |>=mem_top|}
//! @!mem_min=0; {smallest index in \TeX's internal |mem| array;
//!   must be |min_halfword| or more;
//!   must be equal to |mem_bot| in \.{INITEX}, otherwise |<=mem_bot|}
//! @!buf_size=500; {maximum number of characters simultaneously present in
//!   current lines of open files and in control sequences between
//!   \.{\\csname} and \.{\\endcsname}; must not exceed |max_halfword|}
//! @!error_line=72; {width of context lines on terminal error messages}
//! @!half_error_line=42; {width of first lines of contexts in terminal
//!   error messages; should be between 30 and |error_line-15|}
//! @!max_print_line=79; {width of longest text lines output; should be at least 60}
//! @!stack_size=200; {maximum number of simultaneous input sources}
//! @!max_in_open=6; {maximum number of input files and error insertions that
//!   can be going on simultaneously}
//! @!font_max=75; {maximum internal font number; must not exceed |max_quarterword|
//!   and must be at most |font_base+256|}
//! @!font_mem_size=20000; {number of words of |font_info| for all fonts}
//! @!param_size=60; {maximum number of simultaneous macro parameters}
//! @!nest_size=40; {maximum number of semantic levels simultaneously active}
//! @!max_strings=3000; {maximum number of strings; must not exceed |max_halfword|}
//! @!string_vacancies=8000; {the minimum number of characters that should be
//!   available for the user's control sequences and font names,
//!   after \TeX's own error messages are stored}
//! @!pool_size=32000; {maximum number of characters in strings, including all
//!   error messages and help texts, and the names of all fonts and
//!   control sequences; must exceed |string_vacancies| by the total
//!   length of \TeX's own strings, which is currently about 23000}
//! @!save_size=600; {space for saving values outside of current group; must be
//!   at most |max_halfword|}
//! @!trie_size=8000; {space for hyphenation patterns; should be larger for
//!   \.{INITEX} than it is in production versions of \TeX}
//! @!trie_op_size=500; {space for ``opcodes'' in the hyphenation patterns}
//! @!dvi_buf_size=800; {size of the output buffer; must be a multiple of 8}
//! @!file_name_size=40; {file names shouldn't be longer than this}
//! @!pool_name='TeXformats:TEX.POOL                     ';
//!   {string of length |file_name_size|; tells where the string pool appears}
//! @.TeXformats@>
//!
//! @ Like the preceding parameters, the following quantities can be changed
//! at compile time to extend or reduce \TeX's capacity. But if they are changed,
//! it is necessary to rerun the initialization program \.{INITEX}
//! @.INITEX@>
//! to generate new tables for the production \TeX\ program.
//! One can't simply make helter-skelter changes to the following constants,
//! since certain rather complex initialization
//! numbers are computed from them. They are defined here using
//! \.{WEB} macros, instead of being put into \PASCAL's |const| list, in order to
//! emphasize this distinction.
//!
//! @d mem_bot=0 {smallest index in the |mem| array dumped by \.{INITEX};
//!   must not be less than |mem_min|}
//! @d mem_top==30000 {largest index in the |mem| array dumped by \.{INITEX};
//!   must be substantially larger than |mem_bot|
//!   and not greater than |mem_max|}
//! @d font_base=0 {smallest internal font number; must not be less
//!   than |min_quarterword|}
//! @d hash_size=2100 {maximum number of control sequences; it should be at most
//!   about |(mem_max-mem_min)/10|}
//! @d hash_prime=1777 {a prime number equal to about 85\pct! of |hash_size|}
//! @d hyph_size=307 {another prime; the number of \.{\\hyphenation} exceptions}
//! @^system dependencies@>
//!
//! @ In case somebody has inadvertently made bad settings of the ``constants,''
//! \TeX\ checks them using a global variable called |bad|.
//!
//! This is the first of many sections of \TeX\ where global variables are
//! defined.
//!
//! @<Glob...@>=
//! @!bad:integer; {is some ``constant'' wrong?}
//!
//! @ Later on we will say `\ignorespaces|if mem_max>=max_halfword then bad:=14|',
//! or something similar. (We can't do that until |max_halfword| has been defined.)
//!
//! @<Check the ``constant'' values for consistency@>=
//! bad:=0;
//! if (half_error_line<30)or(half_error_line>error_line-15) then bad:=1;
//! if max_print_line<60 then bad:=2;
//! if dvi_buf_size mod 8<>0 then bad:=3;
//! if mem_bot+1100>mem_top then bad:=4;
//! if hash_prime>hash_size then bad:=5;
//! if max_in_open>=128 then bad:=6;
//! if mem_top<256+11 then bad:=7; {we will want |null_list>255|}
//!
//! @ Labels are given symbolic names by the following definitions, so that
//! occasional |goto| statements will be meaningful. We insert the label
//! `|exit|' just before the `\ignorespaces|end|\unskip' of a procedure in
//! which we have used the `|return|' statement defined below; the label
//! `|restart|' is occasionally used at the very beginning of a procedure; and
//! the label `|reswitch|' is occasionally used just prior to a |case|
//! statement in which some cases change the conditions and we wish to branch
//! to the newly applicable case.  Loops that are set up with the |loop|
//! construction defined below are commonly exited by going to `|done|' or to
//! `|found|' or to `|not_found|', and they are sometimes repeated by going to
//! `|continue|'.  If two or more parts of a subroutine start differently but
//! end up the same, the shared code may be gathered together at
//! `|common_ending|'.
//!
//! Incidentally, this program never declares a label that isn't actually used,
//! because some fussy \PASCAL\ compilers will complain about redundant labels.
//!
//! @d exit=10 {go here to leave a procedure}
//! @d restart=20 {go here to start a procedure again}
//! @d reswitch=21 {go here to start a case statement again}
//! @d continue=22 {go here to resume a loop}
//! @d done=30 {go here to exit a loop}
//! @d done1=31 {like |done|, when there is more than one loop}
//! @d done2=32 {for exiting the second loop in a long block}
//! @d done3=33 {for exiting the third loop in a very long block}
//! @d done4=34 {for exiting the fourth loop in an extremely long block}
//! @d done5=35 {for exiting the fifth loop in an immense block}
//! @d done6=36 {for exiting the sixth loop in a block}
//! @d found=40 {go here when you've found it}
//! @d found1=41 {like |found|, when there's more than one per routine}
//! @d found2=42 {like |found|, when there's more than two per routine}
//! @d not_found=45 {go here when you've found nothing}
//! @d common_ending=50 {go here when you want to merge with another branch}
//!
//! @ Here are some macros for common programming idioms.
//!
//! @d incr(#) == #:=#+1 {increase a variable by unity}
//! @d decr(#) == #:=#-1 {decrease a variable by unity}
//! @d negate(#) == #:=-# {change the sign of a variable}
//! @d loop == @+ while true do@+ {repeat over and over until a |goto| happens}
//! @f loop == xclause
//!   {\.{WEB}'s |xclause| acts like `\ignorespaces|while true do|\unskip'}
//! @d do_nothing == {empty statement}
//! @d return == goto exit {terminate a procedure call}
//! @f return == nil
//! @d empty=0 {symbolic name for a null constant}
//!
