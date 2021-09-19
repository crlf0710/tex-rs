//! ` `
// The positioning of accents is straightforward but tedious. Given an accent
// of width |a|, designed for characters of height |x| and slant |s|;
// and given a character of width |w|, height |h|, and slant |t|: We will shift
// the accent down by |x-h|, and we will insert kern nodes that have the effect of
// centering the accent over the character and shifting the accent to the
// right by $\delta={1\over2}(w-a)+h\cdot t-x\cdot s$.  If either character is
// absent from the font, we will simply use the other, without shifting.
//
// @<Declare act...@>=
// procedure make_accent;
#[allow(unused_assignments, unused_variables)]
pub(crate) fn make_accent(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var s,@!t: real; {amount of slant}
    /// amount of slant
    let (s, t): (real, real);
    // @!p,@!q,@!r:pointer; {character, box, and kern nodes}
    /// character, box, and kern nodes
    let (p, mut q, r): (pointer, pointer, pointer);
    // @!f:internal_font_number; {relevant font}
    /// relevant font
    let mut f;
    // @!a,@!h,@!x,@!w,@!delta:scaled; {heights and widths, as explained above}
    // @!i:four_quarters; {character information}
    // begin scan_char_num; f:=cur_font; p:=new_character(f,cur_val);
    scan_char_num(globals, true)?;
    f = cur_font!(globals).into();
    p = new_character(globals, f, ASCII_code::from(globals.cur_val));
    // if p<>null then
    if p != null {
        /// heights and widths, as explained above
        let (a, h, x, w, delta): (scaled, scaled, scaled, scaled, scaled);
        // begin x:=x_height(f); s:=slant(f)/float_constant(65536);
        x = x_height!(globals, f);
        s = slant!(globals, f).inner() as real / float_constant!(65536);
        // @^real division@>
        // a:=char_width(f)(char_info(f)(character(p)));@/
        let p_char_numeric = character!(globals, p).numeric_value();
        a = char_width!(globals, f, char_info!(globals, f, p_char_numeric));
        // do_assignments;@/
        do_assignments(globals)?;
        // @<Create a character node |q| for the next character,
        //   but set |q:=null| if problems arise@>;
        crate::section_1124::Create_a_character_node_q_for_the_next_character__but_set_q_to_null_if_problem_arise!(
            globals, q, f
        );
        // if q<>null then @<Append the accent with appropriate kerns,
        //     then set |p:=q|@>;
        if q != null {
            todo!("q != null");
        }
        // link(tail):=p; tail:=p; space_factor:=1000;
        link!(globals, tail!(globals)) = p;
        tail!(globals) = p;
        space_factor!(globals) = 1000;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::pascal::real;
use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0109::float_constant;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0118::link;
use crate::section_0134::character;
use crate::section_0213::space_factor;
use crate::section_0213::tail;
use crate::section_0230::cur_font;
use crate::section_0434::scan_char_num;
use crate::section_0554::char_info;
use crate::section_0554::char_width;
use crate::section_0558::slant;
use crate::section_0558::x_height;
use crate::section_0582::new_character;
use crate::section_1270::do_assignments;
