//! @ We don't implement \.{\\write} inside of leaders. (The reason is that
//! the number of times a leader box appears might be different in different
//! implementations, due to machine-dependent rounding in the glue calculations.)
//! @^leaders@>
//
// @<Do some work that has been queued up...@>=
pub(crate) macro Do_some_work_that_has_been_queued_up_for_write($globals:expr, $p:expr) {{
    let globals = &mut *$globals;
    // if not doing_leaders then
    if !globals.doing_leaders {
        let p = $p;
        // begin j:=write_stream(p);
        let j = write_stream!(globals, p) as u8;
        // if subtype(p)=write_node then write_out(p)
        if subtype!(globals, p) == write_node {
            write_out(globals, p)?;
        }
        // else  begin if write_open[j] then a_close(write_file[j]);
        else {
            if globals.write_open[j] {
                a_close(&mut globals.write_file[j]);
            }

            // if subtype(p)=close_node then write_open[j]:=false
            if subtype!(globals, p) == close_node {
                globals.write_open[j] = false;
            }
            // else if j<16 then
            else if j < 16 {
                // begin cur_name:=open_name(p); cur_area:=open_area(p);
                globals.cur_name = str_number::new(open_name!(globals, p) as _);
                globals.cur_area = str_number::new(open_area!(globals, p) as _);
                // cur_ext:=open_ext(p);
                globals.cur_ext = str_number::new(open_ext!(globals, p) as _);
                // if cur_ext="" then cur_ext:=".tex";
                if globals.cur_ext == crate::strpool_str!("") {
                    globals.cur_ext = crate::strpool_str!(".tex");
                }
                // pack_cur_name;
                pack_cur_name(globals);
                // while not a_open_out(write_file[j]) do
                while !a_open_out(
                    make_globals_filename_view!(globals),
                    &mut globals.write_file[j],
                ) {
                    // prompt_file_name("output file name",".tex");
                    prompt_file_name(
                        globals,
                        crate::strpool_str!("output file name"),
                        crate::strpool_str!(".tex"),
                    )?;
                }
                // write_open[j]:=true;
                globals.write_open[j] = true;
                // end;
            }
            // end;
        }
        // end
    }
    use crate::pascal::u8_from_0_to_n;
    use crate::section_0004::make_globals_filename_view;
    use crate::section_0004::TeXGlobalsFilenameView;
    use crate::section_0027::a_open_out;
    use crate::section_0028::a_close;
    use crate::section_0038::str_number;
    use crate::section_0133::subtype;
    use crate::section_0529::pack_cur_name;
    use crate::section_0530::prompt_file_name;
    use crate::section_1341::close_node;
    use crate::section_1341::open_area;
    use crate::section_1341::open_ext;
    use crate::section_1341::open_name;
    use crate::section_1341::write_node;
    use crate::section_1341::write_stream;
    use crate::section_1370::write_out;
}}
