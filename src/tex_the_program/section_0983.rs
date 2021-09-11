//! ` `

// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_0983(globals: &mut TeXGlobals) {
    // primitive("pagegoal",set_page_dimen,0);
    primitive(globals, crate::strpool_str!("pagegoal"), set_page_dimen, 0);
    // @!@:page_goal_}{\.{\\pagegoal} primitive@>
    // primitive("pagetotal",set_page_dimen,1);
    primitive(globals, crate::strpool_str!("pagetotal"), set_page_dimen, 1);
    // @!@:page_total_}{\.{\\pagetotal} primitive@>
    // primitive("pagestretch",set_page_dimen,2);
    primitive(
        globals,
        crate::strpool_str!("pagestretch"),
        set_page_dimen,
        2,
    );
    // @!@:page_stretch_}{\.{\\pagestretch} primitive@>
    // primitive("pagefilstretch",set_page_dimen,3);
    primitive(
        globals,
        crate::strpool_str!("pagefilstretch"),
        set_page_dimen,
        3,
    );
    // @!@:page_fil_stretch_}{\.{\\pagefilstretch} primitive@>
    // primitive("pagefillstretch",set_page_dimen,4);
    primitive(
        globals,
        crate::strpool_str!("pagefillstretch"),
        set_page_dimen,
        4,
    );
    // @!@:page_fill_stretch_}{\.{\\pagefillstretch} primitive@>
    // primitive("pagefilllstretch",set_page_dimen,5);
    primitive(
        globals,
        crate::strpool_str!("pagefilllstretch"),
        set_page_dimen,
        5,
    );
    // @!@:page_filll_stretch_}{\.{\\pagefilllstretch} primitive@>
    // primitive("pageshrink",set_page_dimen,6);
    primitive(
        globals,
        crate::strpool_str!("pageshrink"),
        set_page_dimen,
        6,
    );
    // @!@:page_shrink_}{\.{\\pageshrink} primitive@>
    // primitive("pagedepth",set_page_dimen,7);
    primitive(globals, crate::strpool_str!("pagedepth"), set_page_dimen, 7);
    // @!@:page_depth_}{\.{\\pagedepth} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0209::set_page_dimen;
use crate::section_0264::primitive;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
