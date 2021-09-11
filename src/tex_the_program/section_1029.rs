//! @* \[46] The chief executive.
//! We come now to the |main_control| routine, which contains the master
//! switch that causes all the various pieces of \TeX\ to do their things,
//! in the right order.
//!
//! In a sense, this is the grand climax of the program: It applies all the
//! tools that we have worked so hard to construct. In another sense, this is
//! the messiest part of the program: It necessarily refers to other pieces
//! of code all over the place, so that a person can't fully understand what is
//! going on without paging back and forth to be reminded of conventions that
//! are defined elsewhere. We are now at the hub of the web, the central nervous
//! system that touches most of the other parts and ties them together.
//! @^brain@>
//!
//! The structure of |main_control| itself is quite simple. There's a label
//! called |big_switch|, at which point the next token of input is fetched
//! using |get_x_token|. Then the program branches at high speed into one of
//! about 100 possible directions, based on the value of the current
//! mode and the newly fetched command code; the sum |abs(mode)+cur_cmd|
//! indicates what to do next. For example, the case `|vmode+letter|' arises
//! when a letter occurs in vertical mode (or internal vertical mode); this
//! case leads to instructions that initialize a new paragraph and enter
//! horizontal mode.
//!
//! The big |case| statement that contains this multiway switch has been labeled
//! |reswitch|, so that the program can |goto reswitch| when the next token
//! has already been fetched. Most of the cases are quite short; they call
//! an ``action procedure'' that does the work for that case, and then they
//! either |goto reswitch| or they ``fall through'' to the end of the |case|
//! statement, which returns control back to |big_switch|. Thus, |main_control|
//! is not an extremely large procedure, in spite of the multiplicity of things
//! it must do; it is small enough to be handled by \PASCAL\ compilers that put
//! severe restrictions on procedure size.
//! @!@^action procedure@>
//!
//! One case is singled out for special treatment, because it accounts for most
//! of \TeX's activities in typical applications. The process of reading simple
//! text and converting it into |char_node| records, while looking for ligatures
//! and kerns, is part of \TeX's ``inner loop''; the whole program runs
//! efficiently when its inner loop is fast, so this part has been written
//! with particular care.
//!

crate::migration_complete!();
