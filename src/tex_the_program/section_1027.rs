//! @ @<Recover from an unbalanced output routine@>=
//! begin print_err("Unbalanced output routine");
//! @.Unbalanced output routine@>
//! help2("Your sneaky output routine has problematic {'s and/or }'s.")@/
//! ("I can't handle that very well; good luck."); error;
//! repeat get_token;
//! until loc=null;
//! end {loops forever if reading from a file, since |null=min_halfword<=0|}
//!
