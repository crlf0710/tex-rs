//! @ The |hskip| and |vskip| command codes are used for control sequences
//! like \.{\\hss} and \.{\\vfil} as well as for \.{\\hskip} and \.{\\vskip}.
//! The difference is in the value of |cur_chr|.

// @d fil_code=0 {identifies \.{\\hfil} and \.{\\vfil}}
/// identifies `\hfil` and `\vfil`
pub(crate) const fil_code: halfword = 0;
// @d fill_code=1 {identifies \.{\\hfill} and \.{\\vfill}}
/// identifies `\hfill` and `\vfill`
pub(crate) const fill_code: halfword = 1;
// @d ss_code=2 {identifies \.{\\hss} and \.{\\vss}}
/// identifies `\hss` and `\vss`
pub(crate) const ss_code: halfword = 2;
// @d fil_neg_code=3 {identifies \.{\\hfilneg} and \.{\\vfilneg}}
/// identifies `\hfilneg` and `\vfilneg`
pub(crate) const fil_neg_code: halfword = 3;
// @d skip_code=4 {identifies \.{\\hskip} and \.{\\vskip}}
/// identifies `\hskip` and `\vskip`
pub(crate) const skip_code: halfword = 4;
// @d mskip_code=5 {identifies \.{\\mskip}}
/// identifies `\mskip`
pub(crate) const mskip_code: halfword = 5;
//
// @<Put each...@>=
#[distributed_slice(PRIM2HT)]
#[allow(unused_variables)]
pub(crate) fn put_each_of_tex_s_primitivies_into_the_hash_table_1058(globals: &mut TeXGlobals) {
    // primitive("hskip",hskip,skip_code);@/
    primitive(globals, strpool_str!("hskip"), hskip, skip_code);
    // @!@:hskip_}{\.{\\hskip} primitive@>
    // primitive("hfil",hskip,fil_code);
    // @!@:hfil_}{\.{\\hfil} primitive@>
    // primitive("hfill",hskip,fill_code);@/
    // @!@:hfill_}{\.{\\hfill} primitive@>
    // primitive("hss",hskip,ss_code);
    // @!@:hss_}{\.{\\hss} primitive@>
    // primitive("hfilneg",hskip,fil_neg_code);@/
    // @!@:hfil_neg_}{\.{\\hfilneg} primitive@>
    // primitive("vskip",vskip,skip_code);@/
    primitive(globals, strpool_str!("vskip"), vskip, skip_code);
    // @!@:vskip_}{\.{\\vskip} primitive@>
    // primitive("vfil",vskip,fil_code);
    // @!@:vfil_}{\.{\\vfil} primitive@>
    // primitive("vfill",vskip,fill_code);@/
    primitive(globals, strpool_str!("vfill"), vskip, fill_code);
    // @!@:vfill_}{\.{\\vfill} primitive@>
    // primitive("vss",vskip,ss_code);
    // @!@:vss_}{\.{\\vss} primitive@>
    // primitive("vfilneg",vskip,fil_neg_code);@/
    // @!@:vfil_neg_}{\.{\\vfilneg} primitive@>
    // primitive("mskip",mskip,mskip_code);@/
    // @!@:mskip_}{\.{\\mskip} primitive@>
    // primitive("kern",kern,explicit);
    // @!@:kern_}{\.{\\kern} primitive@>
    // primitive("mkern",mkern,mu_glue);@/
    // @!@:mkern_}{\.{\\mkern} primitive@>
}

use crate::section_0004::TeXGlobals;
use crate::section_0208::*;
use crate::section_0264::primitive;
use crate::section_0297::cur_tok_type;
use crate::section_0207::par_end;
use crate::section_1336::PRIM2HT;
use linkme::distributed_slice;

use crate::section_0113::halfword;

// Workaround https://github.com/rust-lang/rust/issues/47384
pub(crate) fn workaround_47384() {}
