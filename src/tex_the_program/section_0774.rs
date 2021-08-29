//! @ When \.{\\halign} or \.{\\valign} has been scanned in an appropriate
//! mode, \TeX\ calls |init_align|, whose task is to get everything off to a
//! good start. This mostly involves scanning the preamble and putting its
//! information into the preamble list.
//! @^preamble@>
//
// @p @t\4@>@<Declare the procedure called |get_preamble_token|@>@t@>@/
// procedure@?align_peek; forward;@t\2@>@/
// procedure@?normal_paragraph; forward;@t\2@>@/
// procedure init_align;
#[allow(unused_variables, unused_assignments)]
pub(crate) fn init_align(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label done, done1, done2, continue;
    // var save_cs_ptr:pointer; {|warning_index| value for error messages}
    /// `warning_index` value for error messages
    let save_cs_ptr: pointer;
    // @!p:pointer; {for short-term temporary use}
    // begin save_cs_ptr:=cur_cs; {\.{\\halign} or \.{\\valign}, usually}
    /// `\halign` or `\valign`, usually
    const _ : () = ();
    save_cs_ptr = globals.cur_cs;
    // push_alignment; align_state:=-1000000; {enter a new alignment level}
    push_alignment(globals)?;
    /// enter a new alignment level
    const _ : () = ();
    globals.align_state = -1000000;
    // @<Check for improper alignment in displayed math@>;
    Check_for_improper_alignment_in_displayed_math!(globals);
    // push_nest; {enter a new semantic level}
    /// enter a new semantic level
    push_nest(globals);
    // @<Change current mode to |-vmode| for \.{\\halign}, |-hmode| for \.{\\valign}@>;
    Change_current_mode_to_neg_vmode_for_halign__neg_hmode_for_valign!(globals);
    // scan_spec(align_group,false);@/
    scan_spec(globals, align_group.into(), false)?;
    // @<Scan the preamble and record it in the |preamble| list@>;
    Scan_the_preamble_and_record_it_in_the_preamble_list!(globals, save_cs_ptr);
    // new_save_level(align_group);
    new_save_level(globals, align_group.into());
    // if every_cr<>null then begin_token_list(every_cr,every_cr_text);
    if every_cr!(globals) != null {
        begin_token_list(globals, every_cr!(globals), every_cr_text);
    }
    // align_peek; {look for \.{\\noalign} or \.{\\omit}}
    /// look for `\noalign` or `\omit`
    align_peek(globals)?;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0216::push_nest;
use crate::section_0269::align_group;
use crate::section_0274::new_save_level;
use crate::section_0307::every_cr_text;
use crate::section_0323::begin_token_list;
use crate::section_0645::scan_spec;
use crate::section_0772::push_alignment;
use crate::section_0785::align_peek;
