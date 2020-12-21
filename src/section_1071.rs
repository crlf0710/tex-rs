//! @ Now let's turn to the question of how \.{\\hbox} is treated. We actually
//! need to consider also a slightly larger context, since constructions like
//! `\.{\\setbox3=}\penalty0\.{\\hbox...}' and
//! `\.{\\leaders}\penalty0\.{\\hbox...}' and
//! `\.{\\lower3.8pt\\hbox...}'
//! are supposed to invoke quite
//! different actions after the box has been packaged. Conversely,
//! constructions like `\.{\\setbox3=}' can be followed by a variety of
//! different kinds of boxes, and we would like to encode such things in an
//! efficient way.
//!
//! In other words, there are two problems: to represent the context of a box,
//! and to represent its type.
//!
//! The first problem is solved by putting a ``context code'' on the |save_stack|,
//! just below the two entries that give the dimensions produced by |scan_spec|.
//! The context code is either a (signed) shift amount, or it is a large
//! integer |>=box_flag|, where |box_flag=@t$2^{30}$@>|. Codes |box_flag| through
//! |box_flag+255| represent `\.{\\setbox0}' through `\.{\\setbox255}';
//! codes |box_flag+256| through |box_flag+511| represent `\.{\\global\\setbox0}'
//! through `\.{\\global\\setbox255}';
//! code |box_flag+512| represents `\.{\\shipout}'; and codes |box_flag+513|
//! through |box_flag+515| represent `\.{\\leaders}', `\.{\\cleaders}',
//! and `\.{\\xleaders}'.
//!
//! The second problem is solved by giving the command code |make_box| to all
//! control sequences that produce a box, and by using the following |chr_code|
//! values to distinguish between them: |box_code|, |copy_code|, |last_box_code|,
//! |vsplit_code|, |vtop_code|, |vtop_code+vmode|, and |vtop_code+hmode|, where
//! the latter two are used to denote \.{\\vbox} and \.{\\hbox}, respectively.
//
// @d box_flag==@'10000000000 {context code for `\.{\\setbox0}'}
/// context code for `\setbox0`
pub(crate) const box_flag: integer = 0o10000000000;
// @d ship_out_flag==box_flag+512 {context code for `\.{\\shipout}'}
// @d leader_flag==box_flag+513 {context code for `\.{\\leaders}'}
/// context code for `\leaders`
pub(crate) const leader_flag: integer = box_flag + 513;
// @d box_code=0 {|chr_code| for `\.{\\box}'}
// @d copy_code=1 {|chr_code| for `\.{\\copy}'}
// @d last_box_code=2 {|chr_code| for `\.{\\lastbox}'}
// @d vsplit_code=3 {|chr_code| for `\.{\\vsplit}'}
// @d vtop_code=4 {|chr_code| for `\.{\\vtop}'}
/// `chr_code` for `\vtop`
pub(crate) const vtop_code: chr_code_repr = 4;

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1071(globals: &mut TeXGlobals) {
    // primitive("moveleft",hmove,1);
    // @!@:move_left_}{\.{\\moveleft} primitive@>
    // primitive("moveright",hmove,0);@/
    // @!@:move_right_}{\.{\\moveright} primitive@>
    // primitive("raise",vmove,1);
    // @!@:raise_}{\.{\\raise} primitive@>
    // primitive("lower",vmove,0);
    // @!@:lower_}{\.{\\lower} primitive@>
    // @#
    // primitive("box",make_box,box_code);
    // @!@:box_}{\.{\\box} primitive@>
    // primitive("copy",make_box,copy_code);
    // @!@:copy_}{\.{\\copy} primitive@>
    // primitive("lastbox",make_box,last_box_code);
    // @!@:last_box_}{\.{\\lastbox} primitive@>
    // primitive("vsplit",make_box,vsplit_code);
    // @!@:vsplit_}{\.{\\vsplit} primitive@>
    // primitive("vtop",make_box,vtop_code);@/
    // @!@:vtop_}{\.{\\vtop} primitive@>
    // primitive("vbox",make_box,vtop_code+vmode);
    // @!@:vbox_}{\.{\\vbox} primitive@>
    // primitive("hbox",make_box,vtop_code+hmode);@/
    primitive(
        globals,
        strpool_str!("hbox"),
        make_box,
        (vtop_code as chr_code_repr + hmode as chr_code_repr) as _,
    );
    // @!@:hbox_}{\.{\\hbox} primitive@>
    // primitive("shipout",leader_ship,a_leaders-1); {|ship_out_flag=leader_flag-1|}
    // @!@:ship_out_}{\.{\\shipout} primitive@>
    // primitive("leaders",leader_ship,a_leaders);
    // @!@:leaders_}{\.{\\leaders} primitive@>
    // primitive("cleaders",leader_ship,c_leaders);
    // @!@:c_leaders_}{\.{\\cleaders} primitive@>
    // primitive("xleaders",leader_ship,x_leaders);
    // @!@:x_leaders_}{\.{\\xleaders} primitive@>
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0208::make_box;
use crate::section_0211::hmode;
use crate::section_0264::primitive;
use crate::section_0297::chr_code_repr;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
