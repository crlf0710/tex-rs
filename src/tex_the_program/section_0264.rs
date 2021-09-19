//! @ We need to put \TeX's ``primitive'' control sequences into the hash
//! table, together with their command code (which will be the |eq_type|)
//! and an operand (which will be the |equiv|). The |primitive| procedure
//! does this, in a way that no \TeX\ user can. The global value |cur_val|
//! contains the new |eqtb| pointer after |primitive| has acted.

// @p @!init procedure primitive(@!s:str_number;@!c:quarterword;@!o:halfword);
#[cfg(feature = "initex")]
#[allow(unused_variables, unreachable_code)]
#[cfg_attr(
    feature = "trace_verbose",
    tracing::instrument(level = "trace", skip(globals))
)]
pub(crate) fn primitive(globals: &mut TeXGlobals, s: str_number, c: quarterword, o: halfword) {
    // var k:pool_pointer; {index into |str_pool|}
    // @!j:small_number; {index into |buffer|}
    // @!l:small_number; {length of the string}
    // begin if s<256 then cur_val:=s+single_base
    if s.0 < 256 {
        globals.cur_val = (s.0.get() as word + single_base) as _;
    }
    // else  begin k:=str_start[s]; l:=str_start[s+1]-k;
    //     {we will move |s| into the (empty) |buffer|}
    else {
        /// we will move `s` into the (empty) `buffer`
        const _: () = ();
        let l: integer;
        // for j:=0 to l-1 do buffer[j]:=so(str_pool[k+j]);
        #[cfg(not(feature = "unicode_support"))]
        {
            let k = globals.str_start[s];
            let l_bytes = globals.str_start[s + 1] - k;
            l = l_bytes;
            for j in 0..=(l - 1) {
                globals.buffer[j] = so(str_pool[k + j]);
            }
        }
        #[cfg(feature = "unicode_support")]
        {
            let mut len = 0;
            for ch in globals.str_pool.str_ascii_codes(&globals.str_start, s) {
                globals.buffer[len] = xord(ch);
                len = len + 1;
            }
            l = len as _;
        }
        // cur_val:=id_lookup(0,l); {|no_new_control_sequence| is |false|}
        /// `no_new_control_sequence` is `false`
        {
            globals.cur_val = id_lookup(globals, 0, l) as _;
        }
        // flush_string; text(cur_val):=s; {we don't want to have the string twice}
        /// we don't want to have the string twice
        {
            flush_string(globals);
            text!(globals, globals.cur_val as pointer) = s.get() as _;
            crate::trace_expr_verbose!("text(cur_val) = text({}) = {}", globals.cur_val, s.get());
        }
        // end;
    }
    // eq_level(cur_val):=level_one; eq_type(cur_val):=c; equiv(cur_val):=o;
    crate::trace_expr_verbose!("cur_val = {}", globals.cur_val);
    eq_level!(globals, globals.cur_val as u32) = level_one;
    eq_type!(globals, globals.cur_val as u32) = c;
    equiv!(globals, globals.cur_val as u32) = o;
    // end;
    // tini
}

use crate::pascal::integer;
use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0020::xord;
use crate::section_0038::str_number;
use crate::section_0044::flush_string;
use crate::section_0113::halfword;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0221::eq_level;
use crate::section_0221::eq_type;
use crate::section_0221::eq_type_field;
use crate::section_0221::equiv;
use crate::section_0221::level_one;
use crate::section_0222::single_base;
use crate::section_0256::text;
use crate::section_0259::id_lookup;
