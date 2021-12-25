//! @ We have saved the worst for last: The fussiest part of math mode processing
//! occurs when a displayed formula is being centered and placed with an optional
//! equation number.
//!
//! @<Local variables for finishing...@>=
//! @!b:pointer; {box containing the equation}
//! @!w:scaled; {width of the equation}
//! @!z:scaled; {width of the line}
//! @!e:scaled; {width of equation number}
//! @!q:scaled; {width of equation number plus space to separate from equation}
//! @!d:scaled; {displacement of equation in the line}
//! @!s:scaled; {move the line right this much}
//! @!g1,@!g2:small_number; {glue parameter codes for before and after}
//! @!r:pointer; {kern node used to position the display}
//! @!t:pointer; {tail of adjustment list}
