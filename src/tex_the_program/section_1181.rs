//! ` `

// @<Declare act...@>=
// procedure math_fraction;
#[allow(unused_variables, unused_assignments)]
pub(crate) fn math_fraction(globals: &mut TeXGlobals) -> TeXResult<()> {
    // var c:small_number; {the type of generalized fraction we are scanning}
    /// the type of generalized fraction we are scanning
    let c;
    // begin c:=cur_chr;
    c = globals.cur_chr.get();
    // if incompleat_noad<>null then
    if incompleat_noad!(globals) != null as _ {
        // @<Ignore the fraction operation and complain about this ambiguous case@>
        todo!("Ignore");
    }
    // else  begin incompleat_noad:=get_node(fraction_noad_size);
    else {
        let incompleat_noad = get_node(globals, fraction_noad_size as _)?;
        incompleat_noad!(globals) = incompleat_noad as _;
        // type(incompleat_noad):=fraction_noad;
        r#type!(globals, incompleat_noad) = fraction_noad;
        // subtype(incompleat_noad):=normal;
        subtype!(globals, incompleat_noad) = noad_subtype::normal as _;
        // math_type(numerator(incompleat_noad)):=sub_mlist;
        math_type!(globals, numerator!(incompleat_noad)) = math_type_kind::sub_mlist as _;
        // info(numerator(incompleat_noad)):=link(head);
        info_inner!(globals, numerator!(incompleat_noad)) = link!(globals, head!(globals));
        // mem[denominator(incompleat_noad)].hh:=empty_field;
        globals.mem[denominator!(incompleat_noad)][MEMORY_WORD_HH] = empty_field;
        // mem[left_delimiter(incompleat_noad)].qqqq:=null_delimiter;
        globals.mem[left_delimiter!(incompleat_noad)][MEMORY_WORD_QQQQ] = null_delimiter;
        // mem[right_delimiter(incompleat_noad)].qqqq:=null_delimiter;@/
        globals.mem[right_delimiter!(incompleat_noad)][MEMORY_WORD_QQQQ] = null_delimiter;
        // link(head):=null; tail:=head;
        link!(globals, head!(globals)) = null;
        tail!(globals) = head!(globals);
        // @<Use code |c| to distinguish between generalized fractions@>;
        crate::section_1182::Use_code_c_to_distinguish_between_generalized_fractions!(globals, c);
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::MEMORY_WORD_QQQQ;
use crate::section_0115::null;
use crate::section_0118::info_inner;
use crate::section_0118::link;
use crate::section_0125::get_node;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0213::head;
use crate::section_0213::incompleat_noad;
use crate::section_0213::tail;
use crate::section_0681::math_type;
use crate::section_0681::math_type_kind;
use crate::section_0683::denominator;
use crate::section_0683::fraction_noad;
use crate::section_0683::fraction_noad_size;
use crate::section_0683::left_delimiter;
use crate::section_0683::numerator;
use crate::section_0683::right_delimiter;
use crate::section_0684::empty_field;
use crate::section_0684::null_delimiter;
use crate::section_0686::noad_subtype;
use crate::tex_the_program::section_0113::MEMORY_WORD_HH;
