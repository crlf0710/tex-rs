#![allow(irrefutable_let_patterns)]

#[path = "auxiliary/vfs.rs"]
mod vfs;

use vfs::prepare_pool;
use vfs::TeXTestVFS;

#[test]
fn initex_immediate_eof() {
    let term_output = prepare_pool().install(|| {
        TeXTestVFS::default().install_as_tex_io_handler();
        if let mut globals = tex::TeXGlobals::default() {
            tex::entry(&mut globals);
        }
        TeXTestVFS::with_current(|vfs| vfs.dump_current_term_out().unwrap())
    });
    assert_eq!(
        concat!(
            "This is TeX-rs, Version 3.141592653 (INITEX)\n",
            "**Please type the name of your input file.\n",
            "**\n",
            "! End of file on the terminal... why?"
        ),
        String::from_utf8_lossy(&term_output).as_ref()
    );
}

#[test]
fn initex_empty_fmt_and_minimal_term_in() {
    let (term_output, empty_log) = prepare_pool().install(|| {
        TeXTestVFS::default()
            .and_then_prepare_file("empty.tex", b"")
            .and_then_prepare_termin(concat!("empty\n", "\\end\n").as_bytes())
            .install_as_tex_io_handler();
        if let mut globals = tex::TeXGlobals::default() {
            tex::entry(&mut globals);
        }
        TeXTestVFS::with_current(|vfs| {
            let term_output = vfs.dump_current_term_out().unwrap();
            let empty_log = vfs.dump_file("empty.log").unwrap();
            (term_output, empty_log)
        })
    });
    assert_eq!(
        concat!(
            "This is TeX-rs, Version 3.141592653 (INITEX)\n",
            "**(empty.tex)\n",
            "*No pages of output.\n",
            "Transcript written on empty.log."
        ),
        String::from_utf8_lossy(&term_output).as_ref()
    );
    assert_eq!(
        concat!(
            "This is TeX-rs, Version 3.141592653 (INITEX)  4 JUL 1776 12:00\n",
            "**empty\n",
            "(empty.tex)\n",
            "*\\end\n",
            "No pages of output.\n",
        ),
        String::from_utf8_lossy(&empty_log).as_ref()
    );
}

const PLAIN_DMP_RECORD: &[u8] = include_bytes!("../tests_data/plain_dmp/plain.fmt");

#[test]
fn initex_plain_dump() {
    let (term_output, plain_log, plain_dmp) = prepare_pool().install(|| {
        TeXTestVFS::default()
            .and_then_prepare_file("plain.tex", include_bytes!("../tests_data/plain/plain.tex"))
            .and_then_prepare_file(
                "hyphen.tex",
                include_bytes!("../tests_data/plain/hyphen.tex"),
            )
            .and_then_prepare_file(
                "manfnt.tfm",
                include_bytes!("../tests_data/plain/manfnt.tfm"),
            )
            .and_then_prepare_file(
                "cmbsy10.tfm",
                include_bytes!("../tests_data/plain/cmbsy10.tfm"),
            )
            .and_then_prepare_file(
                "cmbx10.tfm",
                include_bytes!("../tests_data/plain/cmbx10.tfm"),
            )
            .and_then_prepare_file("cmbx5.tfm", include_bytes!("../tests_data/plain/cmbx5.tfm"))
            .and_then_prepare_file("cmbx6.tfm", include_bytes!("../tests_data/plain/cmbx6.tfm"))
            .and_then_prepare_file("cmbx7.tfm", include_bytes!("../tests_data/plain/cmbx7.tfm"))
            .and_then_prepare_file("cmbx8.tfm", include_bytes!("../tests_data/plain/cmbx8.tfm"))
            .and_then_prepare_file("cmbx9.tfm", include_bytes!("../tests_data/plain/cmbx9.tfm"))
            .and_then_prepare_file(
                "cmcsc10.tfm",
                include_bytes!("../tests_data/plain/cmcsc10.tfm"),
            )
            .and_then_prepare_file(
                "cmdunh10.tfm",
                include_bytes!("../tests_data/plain/cmdunh10.tfm"),
            )
            .and_then_prepare_file(
                "cmex10.tfm",
                include_bytes!("../tests_data/plain/cmex10.tfm"),
            )
            .and_then_prepare_file(
                "cmmi10.tfm",
                include_bytes!("../tests_data/plain/cmmi10.tfm"),
            )
            .and_then_prepare_file("cmmi5.tfm", include_bytes!("../tests_data/plain/cmmi5.tfm"))
            .and_then_prepare_file("cmmi6.tfm", include_bytes!("../tests_data/plain/cmmi6.tfm"))
            .and_then_prepare_file("cmmi7.tfm", include_bytes!("../tests_data/plain/cmmi7.tfm"))
            .and_then_prepare_file("cmmi8.tfm", include_bytes!("../tests_data/plain/cmmi8.tfm"))
            .and_then_prepare_file("cmmi9.tfm", include_bytes!("../tests_data/plain/cmmi9.tfm"))
            .and_then_prepare_file(
                "cmmib10.tfm",
                include_bytes!("../tests_data/plain/cmmib10.tfm"),
            )
            .and_then_prepare_file("cmr10.tfm", include_bytes!("../tests_data/plain/cmr10.tfm"))
            .and_then_prepare_file("cmr5.tfm", include_bytes!("../tests_data/plain/cmr5.tfm"))
            .and_then_prepare_file("cmr6.tfm", include_bytes!("../tests_data/plain/cmr6.tfm"))
            .and_then_prepare_file("cmr7.tfm", include_bytes!("../tests_data/plain/cmr7.tfm"))
            .and_then_prepare_file("cmr8.tfm", include_bytes!("../tests_data/plain/cmr8.tfm"))
            .and_then_prepare_file("cmr9.tfm", include_bytes!("../tests_data/plain/cmr9.tfm"))
            .and_then_prepare_file(
                "cmsl10.tfm",
                include_bytes!("../tests_data/plain/cmsl10.tfm"),
            )
            .and_then_prepare_file("cmsl8.tfm", include_bytes!("../tests_data/plain/cmsl8.tfm"))
            .and_then_prepare_file("cmsl9.tfm", include_bytes!("../tests_data/plain/cmsl9.tfm"))
            .and_then_prepare_file(
                "cmsltt10.tfm",
                include_bytes!("../tests_data/plain/cmsltt10.tfm"),
            )
            .and_then_prepare_file(
                "cmss10.tfm",
                include_bytes!("../tests_data/plain/cmss10.tfm"),
            )
            .and_then_prepare_file(
                "cmssbx10.tfm",
                include_bytes!("../tests_data/plain/cmssbx10.tfm"),
            )
            .and_then_prepare_file(
                "cmssi10.tfm",
                include_bytes!("../tests_data/plain/cmssi10.tfm"),
            )
            .and_then_prepare_file(
                "cmssq8.tfm",
                include_bytes!("../tests_data/plain/cmssq8.tfm"),
            )
            .and_then_prepare_file(
                "cmssqi8.tfm",
                include_bytes!("../tests_data/plain/cmssqi8.tfm"),
            )
            .and_then_prepare_file(
                "cmsy10.tfm",
                include_bytes!("../tests_data/plain/cmsy10.tfm"),
            )
            .and_then_prepare_file("cmsy5.tfm", include_bytes!("../tests_data/plain/cmsy5.tfm"))
            .and_then_prepare_file("cmsy6.tfm", include_bytes!("../tests_data/plain/cmsy6.tfm"))
            .and_then_prepare_file("cmsy7.tfm", include_bytes!("../tests_data/plain/cmsy7.tfm"))
            .and_then_prepare_file("cmsy8.tfm", include_bytes!("../tests_data/plain/cmsy8.tfm"))
            .and_then_prepare_file("cmsy9.tfm", include_bytes!("../tests_data/plain/cmsy9.tfm"))
            .and_then_prepare_file(
                "cmti10.tfm",
                include_bytes!("../tests_data/plain/cmti10.tfm"),
            )
            .and_then_prepare_file("cmti7.tfm", include_bytes!("../tests_data/plain/cmti7.tfm"))
            .and_then_prepare_file("cmti8.tfm", include_bytes!("../tests_data/plain/cmti8.tfm"))
            .and_then_prepare_file("cmti9.tfm", include_bytes!("../tests_data/plain/cmti9.tfm"))
            .and_then_prepare_file(
                "cmtt10.tfm",
                include_bytes!("../tests_data/plain/cmtt10.tfm"),
            )
            .and_then_prepare_file("cmtt8.tfm", include_bytes!("../tests_data/plain/cmtt8.tfm"))
            .and_then_prepare_file("cmtt9.tfm", include_bytes!("../tests_data/plain/cmtt9.tfm"))
            .and_then_prepare_file("cmu10.tfm", include_bytes!("../tests_data/plain/cmu10.tfm"))
            .and_then_prepare_termin(concat!("plain\n", "\\dump\n").as_bytes())
            .install_as_tex_io_handler();
        if let mut globals = tex::TeXGlobals::default() {
            tex::entry(&mut globals);
        }
        TeXTestVFS::with_current(|vfs| {
            let term_output = vfs.dump_current_term_out().unwrap();
            let plain_log = vfs.dump_file("plain.log").unwrap();
            let plain_fmt = vfs.dump_file("plain.fmt").unwrap();
            (term_output, plain_log, plain_fmt)
        })
    });
    assert_eq!(
        String::from_utf8_lossy(&term_output).as_ref(),
        concat!(
            "This is TeX-rs, Version 3.141592653 (INITEX)\n",
            "**(plain.tex Preloading the plain format: codes, registers, parameters, fonts,\n",
            "more fonts, macros, math definitions, output routines, hyphenation (hyphen.tex)\n",
            ")\n",
            "*Beginning to dump on file plain.fmt\n",
            " (preloaded format=plain 1776.7.4)\n",
            "1618 strings of total length 15178\n",
            "4990 memory locations dumped; current usage is 110&4877\n",
            "926 multiletter control sequences\n",
            "\\font\\nullfont=nullfont\n",
            "\\font\\tenrm=cmr10\n",
            "\\font\\preloaded=cmr9\n",
            "\\font\\preloaded=cmr8\n",
            "\\font\\sevenrm=cmr7\n",
            "\\font\\preloaded=cmr6\n",
            "\\font\\fiverm=cmr5\n",
            "\\font\\teni=cmmi10\n",
            "\\font\\preloaded=cmmi9\n",
            "\\font\\preloaded=cmmi8\n",
            "\\font\\seveni=cmmi7\n",
            "\\font\\preloaded=cmmi6\n",
            "\\font\\fivei=cmmi5\n",
            "\\font\\tensy=cmsy10\n",
            "\\font\\preloaded=cmsy9\n",
            "\\font\\preloaded=cmsy8\n",
            "\\font\\sevensy=cmsy7\n",
            "\\font\\preloaded=cmsy6\n",
            "\\font\\fivesy=cmsy5\n",
            "\\font\\tenex=cmex10\n",
            "\\font\\preloaded=cmss10\n",
            "\\font\\preloaded=cmssq8\n",
            "\\font\\preloaded=cmssi10\n",
            "\\font\\preloaded=cmssqi8\n",
            "\\font\\tenbf=cmbx10\n",
            "\\font\\preloaded=cmbx9\n",
            "\\font\\preloaded=cmbx8\n",
            "\\font\\sevenbf=cmbx7\n",
            "\\font\\preloaded=cmbx6\n",
            "\\font\\fivebf=cmbx5\n",
            "\\font\\tentt=cmtt10\n",
            "\\font\\preloaded=cmtt9\n",
            "\\font\\preloaded=cmtt8\n",
            "\\font\\preloaded=cmsltt10\n",
            "\\font\\tensl=cmsl10\n",
            "\\font\\preloaded=cmsl9\n",
            "\\font\\preloaded=cmsl8\n",
            "\\font\\tenit=cmti10\n",
            "\\font\\preloaded=cmti9\n",
            "\\font\\preloaded=cmti8\n",
            "\\font\\preloaded=cmti7\n",
            "\\font\\preloaded=cmu10\n",
            "\\font\\preloaded=cmmib10\n",
            "\\font\\preloaded=cmbsy10\n",
            "\\font\\preloaded=cmcsc10\n",
            "\\font\\preloaded=cmssbx10\n",
            "\\font\\preloaded=cmdunh10\n",
            "\\font\\preloaded=cmr7 at 14.51799pt\n",
            "\\font\\preloaded=cmtt10 at 14.4pt\n",
            "\\font\\preloaded=cmssbx10 at 14.4pt\n",
            "\\font\\preloaded=manfnt\n",
            "14787 words of font info for 50 preloaded fonts\n",
            "14 hyphenation exceptions\n",
            "Hyphenation trie of length 6075 has 181 ops out of 500\n",
            "  181 for language 0\n",
            "No pages of output.\n",
            "Transcript written on plain.log.",
        ),
    );
    assert_eq!(
        String::from_utf8_lossy(&plain_log).as_ref(),
        concat!(
            "This is TeX-rs, Version 3.141592653 (INITEX)  4 JUL 1776 12:00\n",
            "**plain\n",
            "(plain.tex Preloading the plain format: codes, registers,\n",
            "\\maxdimen=\\dimen10\n",
            "\\hideskip=\\skip10\n",
            "\\centering=\\skip11\n",
            "\\p@=\\dimen11\n",
            "\\z@=\\dimen12\n",
            "\\z@skip=\\skip12\n",
            "\\voidb@x=\\box10\n",
            " parameters,\n",
            "\\smallskipamount=\\skip13\n",
            "\\medskipamount=\\skip14\n",
            "\\bigskipamount=\\skip15\n",
            "\\n",
            "ormalbaselineskip=\\skip16\n",
            "\\n",
            "ormallineskip=\\skip17\n",
            "\\n",
            "ormallineskiplimit=\\dimen13\n",
            "\\jot=\\dimen14\n",
            "\\interdisplaylinepenalty=\\count23\n",
            "\\interfootnotelinepenalty=\\count24\n",
            " fonts,\n",
            "more fonts,\n",
            "\\itfam=\\fam4\n",
            "\\slfam=\\fam5\n",
            "\\bffam=\\fam6\n",
            "\\ttfam=\\fam7\n",
            " macros,\n",
            "\\strutbox=\\box11\n",
            "\\mscount=\\count25\n",
            "\\tabs=\\box12\n",
            "\\tabsyet=\\box13\n",
            "\\tabsdone=\\box14\n",
            " math definitions,\n",
            "\\rootbox=\\box15\n",
            "\\p@renwd=\\dimen15\n",
            " output routines,\n",
            "\\headline=\\toks10\n",
            "\\footline=\\toks11\n",
            "\\footins=\\insert254\n",
            "\\topins=\\insert253\n",
            " hyphenation (hyphen.tex))\n",
            "*\\dump\n",
            "Beginning to dump on file plain.fmt\n",
            " (preloaded format=plain 1776.7.4)\n",
            "1618 strings of total length 15178\n",
            "4990 memory locations dumped; current usage is 110&4877\n",
            "926 multiletter control sequences\n",
            "\\font\\n",
            "ullfont=nullfont\n",
            "\\font\\tenrm=cmr10\n",
            "\\font\\preloaded=cmr9\n",
            "\\font\\preloaded=cmr8\n",
            "\\font\\sevenrm=cmr7\n",
            "\\font\\preloaded=cmr6\n",
            "\\font\\fiverm=cmr5\n",
            "\\font\\teni=cmmi10\n",
            "\\font\\preloaded=cmmi9\n",
            "\\font\\preloaded=cmmi8\n",
            "\\font\\seveni=cmmi7\n",
            "\\font\\preloaded=cmmi6\n",
            "\\font\\fivei=cmmi5\n",
            "\\font\\tensy=cmsy10\n",
            "\\font\\preloaded=cmsy9\n",
            "\\font\\preloaded=cmsy8\n",
            "\\font\\sevensy=cmsy7\n",
            "\\font\\preloaded=cmsy6\n",
            "\\font\\fivesy=cmsy5\n",
            "\\font\\tenex=cmex10\n",
            "\\font\\preloaded=cmss10\n",
            "\\font\\preloaded=cmssq8\n",
            "\\font\\preloaded=cmssi10\n",
            "\\font\\preloaded=cmssqi8\n",
            "\\font\\tenbf=cmbx10\n",
            "\\font\\preloaded=cmbx9\n",
            "\\font\\preloaded=cmbx8\n",
            "\\font\\sevenbf=cmbx7\n",
            "\\font\\preloaded=cmbx6\n",
            "\\font\\fivebf=cmbx5\n",
            "\\font\\tentt=cmtt10\n",
            "\\font\\preloaded=cmtt9\n",
            "\\font\\preloaded=cmtt8\n",
            "\\font\\preloaded=cmsltt10\n",
            "\\font\\tensl=cmsl10\n",
            "\\font\\preloaded=cmsl9\n",
            "\\font\\preloaded=cmsl8\n",
            "\\font\\tenit=cmti10\n",
            "\\font\\preloaded=cmti9\n",
            "\\font\\preloaded=cmti8\n",
            "\\font\\preloaded=cmti7\n",
            "\\font\\preloaded=cmu10\n",
            "\\font\\preloaded=cmmib10\n",
            "\\font\\preloaded=cmbsy10\n",
            "\\font\\preloaded=cmcsc10\n",
            "\\font\\preloaded=cmssbx10\n",
            "\\font\\preloaded=cmdunh10\n",
            "\\font\\preloaded=cmr7 at 14.51799pt\n",
            "\\font\\preloaded=cmtt10 at 14.4pt\n",
            "\\font\\preloaded=cmssbx10 at 14.4pt\n",
            "\\font\\preloaded=manfnt\n",
            "14787 words of font info for 50 preloaded fonts\n",
            "14 hyphenation exceptions\n",
            "Hyphenation trie of length 6075 has 181 ops out of 500\n",
            "  181 for language 0\n",
            "No pages of output.\n"
        ),
    );
    let plain_fmt_record = PLAIN_DMP_RECORD;
    assert_eq!(plain_fmt_record, plain_dmp);
}
