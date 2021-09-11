//! @ All the work relating to glue creation has been relegated to the
//! following subroutine. It does not call |build_page|, because it is
//! used in at least one place where that would be a mistake.
//
// @<Declare action...@>=
// procedure append_glue;
#[allow(unused_variables)]
pub(crate) fn append_glue(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var s:small_number; {modifier of skip command}
    /// modifier of skip command
    let s;
    // begin s:=cur_chr;
    s = globals.cur_chr.get() as integer;
    // case s of
    // fil_code: cur_val:=fil_glue;
    if s == fil_code as integer {
        globals.cur_val = fil_glue as _;
    }
    // fill_code: cur_val:=fill_glue;
    else if s == fill_code as integer {
        globals.cur_val = fill_glue as _;
    }
    // ss_code: cur_val:=ss_glue;
    else if s == ss_code as integer {
        globals.cur_val = ss_glue as _;
    }
    // fil_neg_code: cur_val:=fil_neg_glue;
    else if s == fil_neg_code as integer {
        globals.cur_val = fil_neg_glue as _;
    }
    // skip_code: scan_glue(glue_val);
    else if s == skip_code as integer {
        scan_glue(
            globals,
            small_number::new(cur_val_level_kind::glue_val as _),
        )?;
    }
    // mskip_code: scan_glue(mu_val);
    else if s == mskip_code as integer {
        scan_glue(globals, small_number::new(cur_val_level_kind::mu_val as _))?;
    }
    // end; {now |cur_val| points to the glue specification}
    /// now `cur_val` points to the glue specification
    const _: () = ();
    // tail_append(new_glue(cur_val));
    tail_append!(globals, new_glue(globals, globals.cur_val as pointer)?);
    // if s>=skip_code then
    if s >= skip_code as integer {
        // begin decr(glue_ref_count(cur_val));
        decr!(glue_ref_count!(globals, globals.cur_val as pointer));
        // if s>skip_code then subtype(tail):=mu_glue;
        if s > skip_code as integer {
            subtype!(globals, tail!(globals)) = glue_node_subtype::mu_glue as _;
        }
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0115::pointer;
use crate::section_0133::subtype;
use crate::section_0149::glue_node_subtype;
use crate::section_0150::glue_ref_count;
use crate::section_0153::new_glue;
use crate::section_0162::fil_glue;
use crate::section_0162::fil_neg_glue;
use crate::section_0162::fill_glue;
use crate::section_0162::ss_glue;
use crate::section_0213::tail;
use crate::section_0214::tail_append;
use crate::section_0410::cur_val_level_kind;
use crate::section_0461::scan_glue;
use crate::section_1058::fil_code;
use crate::section_1058::fil_neg_code;
use crate::section_1058::fill_code;
use crate::section_1058::mskip_code;
use crate::section_1058::skip_code;
use crate::section_1058::ss_code;
