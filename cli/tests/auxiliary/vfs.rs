use std::cell::RefCell;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use vfs_crate::VfsPath;

pub(crate) struct TeXTestVFS {
    disk_vfs: VfsPath,
    term_vfs: VfsPath,
    term_idx: AtomicUsize,
}

thread_local! {
    static VFS: RefCell<TeXTestVFS> = RefCell::new(TeXTestVFS::default());
}

impl Default for TeXTestVFS {
    fn default() -> Self {
        let disk_vfs = VfsPath::new(vfs_crate::MemoryFS::new());
        let term_vfs = VfsPath::new(vfs_crate::MemoryFS::new());
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

    pub(crate) fn dump_current_term_out(&self) -> Result<Vec<u8>, usize> {
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

    pub(crate) fn dump_file(&self, path: &str) -> Result<Vec<u8>, usize> {
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

    pub(crate) fn and_then_prepare_termin(self, data: &[u8]) -> Self {
        self.prepare_termin(data);
        self
    }

    pub(crate) fn prepare_termin(&self, data: &[u8]) {
        let idx = self.term_idx.load(Ordering::SeqCst);
        let term_in = format!("termin.{}.txt", idx);
        let root = &self.term_vfs;
        root.join(&term_in)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(data)
            .unwrap();
    }

    pub(crate) fn and_then_prepare_file(self, path: &str, data: &[u8]) -> Self {
        self.prepare_file(path, data);
        self
    }

    pub(crate) fn prepare_file(&self, path: &str, data: &[u8]) {
        let root = &self.disk_vfs;
        root.join(path)
            .unwrap()
            .create_file()
            .unwrap()
            .write_all(data)
            .unwrap();
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

pub(crate) fn prepare_pool() -> rayon::ThreadPool {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    pool
}
