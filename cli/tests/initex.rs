#![allow(irrefutable_let_patterns)]

use std::cell::RefCell;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use vfs::VfsPath;

struct TeXTestVFS {
    disk_vfs: VfsPath,
    term_vfs: VfsPath,
    term_idx: AtomicUsize,
}

thread_local! {
    static VFS: RefCell<TeXTestVFS> = RefCell::new(TeXTestVFS::default());
}

impl Default for TeXTestVFS {
    fn default() -> Self {
        let disk_vfs = VfsPath::new(vfs::MemoryFS::new());
        let term_vfs = VfsPath::new(vfs::MemoryFS::new());
        let term_idx = AtomicUsize::new(0);
        let vfs = TeXTestVFS {
            disk_vfs,
            term_vfs,
            term_idx,
        };
        vfs.init_current_term();
        vfs
    }
}

impl TeXTestVFS {
    fn init_current_term(&self) {
        let idx = self.term_idx.load(Ordering::SeqCst);
        let term_in = format!("termin.{}.txt", idx);
        let term_out = format!("termout.{}.txt", idx);
        let root = &self.term_vfs;
        root.join(&term_in).unwrap().create_file().unwrap();
        root.join(&term_out).unwrap().create_file().unwrap();
    }

    fn open_current_term_in(&self) -> Result<Box<dyn tex::TeXIoReadLine>, usize> {
        let idx = self.term_idx.load(Ordering::SeqCst);
        let term_in = format!("termin.{}.txt", idx);
        let root = &self.term_vfs;
        let data = root
            .join(&term_in)
            .unwrap()
            .open_file()
            .map_err(|_| 1usize)?;
        Ok(Box::new(io::BufReader::new(data)))
    }

    fn open_current_term_out(&self) -> Result<Box<dyn io::Write>, usize> {
        let idx = self.term_idx.load(Ordering::SeqCst);
        let term_out = format!("termout.{}.txt", idx);
        let root = &self.term_vfs;
        root.join(&term_out).unwrap().create_file().map_err(|_| 1)
    }

    fn dump_current_term_out(&self) -> Result<Vec<u8>, usize> {
        let idx = self.term_idx.load(Ordering::SeqCst);
        let term_out = format!("termout.{}.txt", idx);
        let root = &self.term_vfs;
        let mut data = root
            .join(&term_out)
            .unwrap()
            .open_file()
            .map_err(|_| 1usize)?;
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer).map_err(|_| 1usize)?;
        Ok(buffer)
    }

    fn dump_file(&self, path: &str) -> Result<Vec<u8>, usize> {
        let root = &self.disk_vfs;
        let mut data = root.join(path).unwrap().open_file().map_err(|_| 1usize)?;
        let mut buffer = Vec::new();
        data.read_to_end(&mut buffer).map_err(|_| 1usize)?;
        Ok(buffer)
    }

    fn open_text_file(&self, path: &str) -> Result<Box<dyn tex::TeXIoReadLine>, usize> {
        let root = &self.disk_vfs;
        let data = root.join(path).unwrap().open_file().map_err(|_| 1usize)?;
        Ok(Box::new(io::BufReader::new(data)))
    }

    fn open_binary_file(&self, path: &str) -> Result<Box<dyn io::Read>, usize> {
        let root = &self.disk_vfs;
        let data = root.join(path).unwrap().open_file().map_err(|_| 1usize)?;
        Ok(Box::new(data))
    }

    fn create_file(&self, path: &str) -> Result<Box<dyn io::Write>, usize> {
        let root = &self.disk_vfs;
        let data = root.join(path).unwrap().create_file().map_err(|_| 1usize)?;
        Ok(data)
    }

    fn prepare_termin(self, data: &[u8]) -> Self {
        let idx = self.term_idx.load(Ordering::SeqCst);
        let term_in = format!("termin.{}.txt", idx);
        let root = &self.term_vfs;
        root.join(&term_in)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(data)
            .unwrap();
        self
    }

    fn prepare_file(self, path: &str, data: &[u8]) -> Self {
        let root = &self.disk_vfs;
        root.join(path)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(data)
            .unwrap();
        self
    }

    pub(crate) fn with_current<OP, R>(f: OP) -> R
    where
        OP: FnOnce(&Self) -> R,
    {
        VFS.with(|vfs| f(&vfs.borrow()))
    }

    pub(crate) fn install_as_tex_io_handler(self) {
        VFS.with(|vfs| {
            *vfs.borrow_mut() = self;
        });
        tex::install_io_handler(tex::TeXIoHandler {
            open_text_file_for_read: Box::new(|path| {
                VFS.with(|vfs| {
                    let vfs = vfs.borrow();
                    if path == "TTY:" {
                        let read = vfs.open_current_term_in()?;
                        Ok((read, true))
                    } else {
                        let read = vfs.open_text_file(path)?;
                        Ok((read, false))
                    }
                })
            }),
            open_binary_file_for_read: Box::new(|mut path| {
                VFS.with(|vfs| {
                    let vfs = vfs.borrow();
                    if path == "TTY:" {
                        unreachable!()
                    } else {
                        path = path.trim_start_matches("TeXfonts:");
                        path = path.trim_start_matches("TeXformats:");
                        vfs.open_binary_file(path)
                    }
                })
            }),
            open_file_for_write: Box::new(|mut path| {
                VFS.with(|vfs| {
                    let vfs = vfs.borrow();
                    if path == "TTY:" {
                        vfs.open_current_term_out()
                    } else {
                        path = path.trim_start_matches("TeXfonts:");
                        path = path.trim_start_matches("TeXformats:");
                        vfs.create_file(path)
                    }
                })
            }),
        })
    }
}

fn prepare_pool() -> rayon::ThreadPool {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    pool
}

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
            .prepare_file("empty.tex", b"")
            .prepare_termin(b"empty\n\\end\n")
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

#[test]
fn initex_plain_dump() {
    let (term_output, plain_log, plain_dmp) = prepare_pool().install(|| {
        TeXTestVFS::default()
            .prepare_file("plain.tex", include_bytes!("../tests_data/plain/plain.tex"))
            .prepare_file(
                "hyphen.tex",
                include_bytes!("../tests_data/plain/hyphen.tex"),
            )
            .prepare_file(
                "manfnt.tfm",
                include_bytes!("../tests_data/plain/manfnt.tfm"),
            )
            .prepare_file(
                "cmbsy10.tfm",
                include_bytes!("../tests_data/plain/cmbsy10.tfm"),
            )
            .prepare_file(
                "cmbx10.tfm",
                include_bytes!("../tests_data/plain/cmbx10.tfm"),
            )
            .prepare_file("cmbx5.tfm", include_bytes!("../tests_data/plain/cmbx5.tfm"))
            .prepare_file("cmbx6.tfm", include_bytes!("../tests_data/plain/cmbx6.tfm"))
            .prepare_file("cmbx7.tfm", include_bytes!("../tests_data/plain/cmbx7.tfm"))
            .prepare_file("cmbx8.tfm", include_bytes!("../tests_data/plain/cmbx8.tfm"))
            .prepare_file("cmbx9.tfm", include_bytes!("../tests_data/plain/cmbx9.tfm"))
            .prepare_file(
                "cmcsc10.tfm",
                include_bytes!("../tests_data/plain/cmcsc10.tfm"),
            )
            .prepare_file(
                "cmdunh10.tfm",
                include_bytes!("../tests_data/plain/cmdunh10.tfm"),
            )
            .prepare_file(
                "cmex10.tfm",
                include_bytes!("../tests_data/plain/cmex10.tfm"),
            )
            .prepare_file(
                "cmmi10.tfm",
                include_bytes!("../tests_data/plain/cmmi10.tfm"),
            )
            .prepare_file("cmmi5.tfm", include_bytes!("../tests_data/plain/cmmi5.tfm"))
            .prepare_file("cmmi6.tfm", include_bytes!("../tests_data/plain/cmmi6.tfm"))
            .prepare_file("cmmi7.tfm", include_bytes!("../tests_data/plain/cmmi7.tfm"))
            .prepare_file("cmmi8.tfm", include_bytes!("../tests_data/plain/cmmi8.tfm"))
            .prepare_file("cmmi9.tfm", include_bytes!("../tests_data/plain/cmmi9.tfm"))
            .prepare_file(
                "cmmib10.tfm",
                include_bytes!("../tests_data/plain/cmmib10.tfm"),
            )
            .prepare_file("cmr10.tfm", include_bytes!("../tests_data/plain/cmr10.tfm"))
            .prepare_file("cmr5.tfm", include_bytes!("../tests_data/plain/cmr5.tfm"))
            .prepare_file("cmr6.tfm", include_bytes!("../tests_data/plain/cmr6.tfm"))
            .prepare_file("cmr7.tfm", include_bytes!("../tests_data/plain/cmr7.tfm"))
            .prepare_file("cmr8.tfm", include_bytes!("../tests_data/plain/cmr8.tfm"))
            .prepare_file("cmr9.tfm", include_bytes!("../tests_data/plain/cmr9.tfm"))
            .prepare_file(
                "cmsl10.tfm",
                include_bytes!("../tests_data/plain/cmsl10.tfm"),
            )
            .prepare_file("cmsl8.tfm", include_bytes!("../tests_data/plain/cmsl8.tfm"))
            .prepare_file("cmsl9.tfm", include_bytes!("../tests_data/plain/cmsl9.tfm"))
            .prepare_file(
                "cmsltt10.tfm",
                include_bytes!("../tests_data/plain/cmsltt10.tfm"),
            )
            .prepare_file(
                "cmss10.tfm",
                include_bytes!("../tests_data/plain/cmss10.tfm"),
            )
            .prepare_file(
                "cmssbx10.tfm",
                include_bytes!("../tests_data/plain/cmssbx10.tfm"),
            )
            .prepare_file(
                "cmssi10.tfm",
                include_bytes!("../tests_data/plain/cmssi10.tfm"),
            )
            .prepare_file(
                "cmssq8.tfm",
                include_bytes!("../tests_data/plain/cmssq8.tfm"),
            )
            .prepare_file(
                "cmssqi8.tfm",
                include_bytes!("../tests_data/plain/cmssqi8.tfm"),
            )
            .prepare_file(
                "cmsy10.tfm",
                include_bytes!("../tests_data/plain/cmsy10.tfm"),
            )
            .prepare_file("cmsy5.tfm", include_bytes!("../tests_data/plain/cmsy5.tfm"))
            .prepare_file("cmsy6.tfm", include_bytes!("../tests_data/plain/cmsy6.tfm"))
            .prepare_file("cmsy7.tfm", include_bytes!("../tests_data/plain/cmsy7.tfm"))
            .prepare_file("cmsy8.tfm", include_bytes!("../tests_data/plain/cmsy8.tfm"))
            .prepare_file("cmsy9.tfm", include_bytes!("../tests_data/plain/cmsy9.tfm"))
            .prepare_file(
                "cmti10.tfm",
                include_bytes!("../tests_data/plain/cmti10.tfm"),
            )
            .prepare_file("cmti7.tfm", include_bytes!("../tests_data/plain/cmti7.tfm"))
            .prepare_file("cmti8.tfm", include_bytes!("../tests_data/plain/cmti8.tfm"))
            .prepare_file("cmti9.tfm", include_bytes!("../tests_data/plain/cmti9.tfm"))
            .prepare_file(
                "cmtt10.tfm",
                include_bytes!("../tests_data/plain/cmtt10.tfm"),
            )
            .prepare_file("cmtt8.tfm", include_bytes!("../tests_data/plain/cmtt8.tfm"))
            .prepare_file("cmtt9.tfm", include_bytes!("../tests_data/plain/cmtt9.tfm"))
            .prepare_file("cmu10.tfm", include_bytes!("../tests_data/plain/cmu10.tfm"))
            .prepare_termin(b"plain\n\\dump\n")
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
        concat!(
            "This is TeX-rs, Version 3.141592653 (INITEX)\n",
            "**(plain.tex Preloading the plain format: codes, registers, parameters, fonts,\n",
            "more fonts, macros, math definitions, output routines, hyphenation (hyphen.tex)\n",
            ")\n",
            "*Beginning to dump on file plain.fmt\n",
            " (preloaded format=plain 1776.7.4)\n",
            "1562 strings of total length 14305\n",
            "4990 memory locations dumped; current usage is 110&4877\n",
            "894 multiletter control sequences\n",
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
        String::from_utf8_lossy(&term_output).as_ref()
    );
    assert_eq!(
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
            "1562 strings of total length 14305\n",
            "4990 memory locations dumped; current usage is 110&4877\n",
            "894 multiletter control sequences\n",
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
        String::from_utf8_lossy(&plain_log).as_ref()
    );
    let plain_fmt_record = &include_bytes!("../tests_data/plain_dmp/plain.fmt")[..];
    assert_eq!(plain_fmt_record, plain_dmp);
}
