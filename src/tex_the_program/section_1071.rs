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
/// context code for `\shipout`
pub(crate) const ship_out_flag: integer = box_flag + 512;
// @d leader_flag==box_flag+513 {context code for `\.{\\leaders}'}
/// context code for `\leaders`
pub(crate) const leader_flag: integer = box_flag + 513;
// @d box_code=0 {|chr_code| for `\.{\\box}'}
/// `chr_code` for `\box'
pub(crate) const box_code: chr_code_repr = 0;
// @d copy_code=1 {|chr_code| for `\.{\\copy}'}
/// `chr_code` for `\copy`
pub(crate) const copy_code: chr_code_repr = 1;
// @d last_box_code=2 {|chr_code| for `\.{\\lastbox}'}
/// `chr_code` for `\lastbox`
pub(crate) const last_box_code: chr_code_repr = 2;
// @d vsplit_code=3 {|chr_code| for `\.{\\vsplit}'}
/// `chr_code` for `\vsplit`
pub(crate) const vsplit_code: chr_code_repr = 3;
// @d vtop_code=4 {|chr_code| for `\.{\\vtop}'}
/// `chr_code` for `\vtop`
pub(crate) const vtop_code: chr_code_repr = 4;

// @<Put each...@>=
#[allow(unused_variables)]
pub(crate) macro Put_each_of_tex_s_primitivies_into_the_hash_table_1071($globals:expr) {{
    let globals = &mut *$globals;
    // primitive("moveleft",hmove,1);
    primitive(globals, crate::strpool_str!("moveleft"), hmove, 1);
    // @!@:move_left_}{\.{\\moveleft} primitive@>
    // primitive("moveright",hmove,0);@/
    primitive(globals, crate::strpool_str!("moveright"), hmove, 0);
    // @!@:move_right_}{\.{\\moveright} primitive@>
    // primitive("raise",vmove,1);
    primitive(globals, crate::strpool_str!("raise"), vmove, 1);
    // @!@:raise_}{\.{\\raise} primitive@>
    // primitive("lower",vmove,0);
    primitive(globals, crate::strpool_str!("lower"), vmove, 0);
    // @!@:lower_}{\.{\\lower} primitive@>
    // @#
    // primitive("box",make_box,box_code);
    primitive(globals, crate::strpool_str!("box"), make_box, box_code as _);
    // @!@:box_}{\.{\\box} primitive@>
    // primitive("copy",make_box,copy_code);
    primitive(
        globals,
        crate::strpool_str!("copy"),
        make_box,
        copy_code as _,
    );
    // @!@:copy_}{\.{\\copy} primitive@>
    // primitive("lastbox",make_box,last_box_code);
    primitive(
        globals,
        crate::strpool_str!("lastbox"),
        make_box,
        last_box_code as _,
    );
    // @!@:last_box_}{\.{\\lastbox} primitive@>
    // primitive("vsplit",make_box,vsplit_code);
    primitive(
        globals,
        crate::strpool_str!("vsplit"),
        make_box,
        vsplit_code as _,
    );
    // @!@:vsplit_}{\.{\\vsplit} primitive@>
    // primitive("vtop",make_box,vtop_code);@/
    primitive(
        globals,
        crate::strpool_str!("vtop"),
        make_box,
        (vtop_code as chr_code_repr) as _,
    );
    // @!@:vtop_}{\.{\\vtop} primitive@>
    // primitive("vbox",make_box,vtop_code+vmode);
    primitive(
        globals,
        crate::strpool_str!("vbox"),
        make_box,
        (vtop_code as chr_code_repr + vmode as chr_code_repr) as _,
    );
    // @!@:vbox_}{\.{\\vbox} primitive@>
    // primitive("hbox",make_box,vtop_code+hmode);@/
    primitive(
        globals,
        crate::strpool_str!("hbox"),
        make_box,
        (vtop_code as chr_code_repr + hmode as chr_code_repr) as _,
    );
    // @!@:hbox_}{\.{\\hbox} primitive@>
    // primitive("shipout",leader_ship,a_leaders-1); {|ship_out_flag=leader_flag-1|}
    /// `ship_out_flag=leader_flag-1`
    const _: () = ();
    primitive(
        globals,
        crate::strpool_str!("shipout"),
        leader_ship,
        glue_node_subtype::a_leaders as halfword - 1,
    );
    // @!@:ship_out_}{\.{\\shipout} primitive@>
    // primitive("leaders",leader_ship,a_leaders);
    primitive(
        globals,
        crate::strpool_str!("leaders"),
        leader_ship,
        glue_node_subtype::a_leaders as _,
    );
    // @!@:leaders_}{\.{\\leaders} primitive@>
    // primitive("cleaders",leader_ship,c_leaders);
    primitive(
        globals,
        crate::strpool_str!("cleaders"),
        leader_ship,
        glue_node_subtype::c_leaders as _,
    );
    // @!@:c_leaders_}{\.{\\cleaders} primitive@>
    // primitive("xleaders",leader_ship,x_leaders);
    primitive(
        globals,
        crate::strpool_str!("xleaders"),
        leader_ship,
        glue_node_subtype::x_leaders as _,
    );
    // @!@:x_leaders_}{\.{\\xleaders} primitive@>
}}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0113::halfword;
use crate::section_0149::glue_node_subtype;
use crate::section_0208::*;
use crate::section_0211::hmode;
use crate::section_0211::vmode;
use crate::section_0264::primitive;
use crate::section_0297::chr_code_repr;
