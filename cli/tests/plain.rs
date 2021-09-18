#![allow(irrefutable_let_patterns)]

#[path = "auxiliary/vfs.rs"]
mod vfs;

use vfs::prepare_pool;
use vfs::TeXTestVFS;

const PLAIN_DMP_RECORD: &[u8] = include_bytes!("../tests_data/plain_dmp/plain.fmt");

#[test]
fn plain_hello_world() {
    let (term_output, texput_log, texput_dvi) = prepare_pool().install(|| {
        TeXTestVFS::default()
            .prepare_file("plain.fmt", PLAIN_DMP_RECORD)
            .prepare_termin(concat!("&plain\n", "Hello world\n", "\\end\n").as_bytes())
            .install_as_tex_io_handler();
        if let mut globals = tex::TeXGlobals::default() {
            tex::entry(&mut globals);
        }
        TeXTestVFS::with_current(|vfs| {
            let term_output = vfs.dump_current_term_out().unwrap();
            let texput_log = vfs.dump_file("texput.log").unwrap();
            let texput_dvi = vfs.dump_file("texput.dvi").unwrap();
            (term_output, texput_log, texput_dvi)
        })
    });
    assert_eq!(
        concat!(
            "This is TeX-rs, Version 3.141592653 (INITEX)\n",
            "**\n",
            "*\n",
            "*[1]\n",
            "Output written on texput.dvi (1 page, 224 bytes).\n",
            "Transcript written on texput.log."
        ),
        String::from_utf8_lossy(&term_output).as_ref()
    );
    assert_eq!(
        concat!(
            "This is TeX-rs, Version 3.141592653 (preloaded format=plain 1776.7.4)  4 JUL 1776 12:00\n",
            "**&plain\n",
            "\n",
            "*Hello world\n",
            "\n",
            "*\\end\n",
            "[1]\n",
            "Output written on texput.dvi (1 page, 224 bytes).\n",
        ),
        String::from_utf8_lossy(&texput_log).as_ref()
    );
    let plain_helloworld_dvi_record =
        include_bytes!("../tests_data/dvi_result/plain_helloworld.dvi");
    assert_eq!(plain_helloworld_dvi_record, &texput_dvi[..]);
}
