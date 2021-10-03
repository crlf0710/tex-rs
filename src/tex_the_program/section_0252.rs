//! @ Here is a procedure that displays the contents of |eqtb[n]|
//! symbolically.
//
// @p@t\4@>@<Declare the procedure called |print_cmd_chr|@>@;@/
// @!stat procedure show_eqtb(@!n:pointer);
#[cfg(feature = "statistics")]
pub(crate) fn show_eqtb(globals: &mut TeXGlobals, n: pointer) {
    // begin if n<active_base then print_char("?") {this can't happen}
    if (n as integer) < active_base as integer {
        /// this can't happen
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b'?'),
        );
    }
    // else if n<glue_base then @<Show equivalent |n|, in region 1 or 2@>
    else if (n as integer) < glue_base as integer {
        todo!("show n in region 1 or 2");
    }
    // else if n<local_base then @<Show equivalent |n|, in region 3@>
    else if (n as integer) < local_base as integer {
        crate::section_0229::Show_equivalent_n__in_region_3!(globals, n);
    }
    // else if n<int_base then @<Show equivalent |n|, in region 4@>
    else if (n as integer) < int_base as integer {
        crate::section_0233::Show_equivalent_n__in_region_4!(globals, n);
    }
    // else if n<dimen_base then @<Show equivalent |n|, in region 5@>
    else if (n as integer) < dimen_base as integer {
        todo!("show n in region 5");
    }
    // else if n<=eqtb_size then @<Show equivalent |n|, in region 6@>
    else if (n as integer) < eqtb_size as integer {
        crate::section_0251::Show_equivalent_n__in_region_6!(globals, n);
    }
    // else print_char("?"); {this can't happen either}
    else {
        /// this can't happen either
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b'?'),
        );
    }
    // end;
    // tats
}

use crate::pascal::integer;
use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0018::ASCII_code_literal;
use crate::section_0058::print_char;
use crate::section_0115::pointer;
use crate::section_0222::active_base;
use crate::section_0222::glue_base;
use crate::section_0224::local_base;
use crate::section_0230::int_base;
use crate::section_0236::dimen_base;
use crate::section_0247::eqtb_size;
