//! ` `

// @<Declare subprocedures for |prefixed_command|@>=
// procedure new_font(@!a:small_number);
#[allow(unused_variables, unused_assignments)]
pub(crate) fn new_font(globals: &mut TeXGlobals, a: small_number) -> TeXResult<()> {
    // label common_ending;
    // var u:pointer; {user's font identifier}
    /// user's font identifier
    let u: pointer;
    // @!s:scaled; {stated ``at'' size, or negative of scaled magnification}
    /// stated "at" size, or negative of scaled magnification
    let s: scaled;
    // @!f:internal_font_number; {runs through existing fonts}
    /// runs through existing fonts
    let f: internal_font_number;
    // @!t:str_number; {name for the frozen font identifier}
    /// name for the frozen font identifier
    let t: str_number;
    // @!old_setting:0..max_selector; {holds |selector| setting}
    /// holds |selector| setting
    let old_setting;
    // @!flushable_string:str_number; {string not yet referenced}
    // begin if job_name=0 then open_log_file;
    //   {avoid confusing \.{texput} with the font name}
    /// avoid confusing `texput` with the font name
    if globals.job_name == 0 {
        open_log_file(globals);
    }
    // @.texput@>
    // get_r_token; u:=cur_cs;
    get_r_token(globals)?;
    u = globals.cur_cs;
    // if u>=hash_base then t:=text(u)
    if u >= hash_base as pointer {
        t = str_number::new(text!(globals, u) as _);
    }
    // else if u>=single_base then
    else if u >= single_base as pointer {
        // if u=null_cs then t:="FONT"@+else t:=u-single_base
        if u == null_cs {
            t = strpool_str!("FONT");
        } else {
            t = str_number::new((u - single_base as pointer) as _);
        }
    }
    // else  begin old_setting:=selector; selector:=new_string;
    else {
        old_setting = globals.selector;
        globals.selector = new_string.into();
        // print("FONT"); print(u-active_base); selector:=old_setting;
        print(globals, strpool_str!("FONT").get() as _);
        print(globals, (u - active_base as pointer) as _);
        globals.selector = old_setting;
        // @.FONTx@>
        // str_room(1); t:=make_string;
        str_room(globals, 1 * character_max_room);
        t = make_string(make_globals_string_view!(globals));
        // end;
    }
    // define(u,set_font,null_font); scan_optional_equals; scan_file_name;
    define!(globals, a, u, set_font, null_font as _);
    scan_optional_equals(globals)?;
    scan_file_name(globals)?;
    // @<Scan the font size specification@>;
    Scan_the_font_size_specification!(globals, s);
    // @<If this font has already been loaded, set |f| to the internal
    //   font number and |goto common_ending|@>;
    If_this_font_has_already_been_loaded_set_f_to_the_internal_font_number_and_goto_common_ending!
        (globals, s);
    // f:=read_font_info(u,cur_name,cur_area,s);
    f = read_font_info(globals, u, globals.cur_name, globals.cur_area, s)?;
    // common_ending: equiv(u):=f; eqtb[font_id_base+f]:=eqtb[u]; font_id_text(f):=t;
    equiv!(globals, u) = f.get();
    globals.eqtb[(font_id_base + f.get() as word) as pointer] = globals.eqtb[u];
    font_id_text!(globals, f.get() as word) = t.get() as _;
    // end;
    ok_nojump!()
}

use crate::pascal::word;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsStringView;
use crate::section_0038::str_number;
use crate::section_0042::str_room;
use crate::section_0042::character_max_room;
use crate::section_0043::make_string;
use crate::section_0054::new_string;
use crate::section_0059::print;
use crate::section_0081::TeXResult;
use crate::section_0101::small_number;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0209::set_font;
use crate::section_0222::null_cs;
use crate::section_0222::single_base;
use crate::section_0222::hash_base;
use crate::section_0222::active_base;
use crate::section_0232::null_font;
use crate::section_0222::font_id_base;
use crate::section_0405::scan_optional_equals;
use crate::section_0526::scan_file_name;
use crate::section_0534::open_log_file;
use crate::section_0548::internal_font_number;
use crate::section_0560::read_font_info;
use crate::section_1215::get_r_token;
