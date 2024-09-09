use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
struct System {
    root: Rc<RefCell<dyn Folder>>,
}

trait Folder: std::fmt::Debug {
    fn subdir(&mut self, sys: &mut System, name: &str) -> Option<Rc<RefCell<dyn Folder>>>;
}

#[derive(Debug)]
struct Realdir {
    subdirs: BTreeMap<String, Rc<RefCell<dyn Folder>>>,
}

#[derive(Debug)]
struct Dynadir {}

impl Folder for Realdir {
    fn subdir(&mut self, _sys: &mut System, name: &str) -> Option<Rc<RefCell<dyn Folder>>> {
        Some(
            self.subdirs
                .entry(name.to_string())
                .or_insert_with(|| {
                    if name.to_string() == "dynamic" {
                        Rc::new(RefCell::new(Dynadir::new()))
                    } else {
                        Rc::new(RefCell::new(Realdir::new()))
                    }
                })
                .clone(),
        )
    }
}

impl Folder for Dynadir {
    fn subdir(&mut self, sys: &mut System, _name: &str) -> Option<Rc<RefCell<dyn Folder>>> {
        Some(sys.wd().lookup(Path::new("d/e/f")))
    }
}

impl Realdir {
    fn new() -> Realdir {
        Realdir {
            subdirs: BTreeMap::new(),
        }
    }
}

impl Dynadir {
    fn new() -> Dynadir {
        Dynadir {}
    }
}

impl<'a> System {
    fn new() -> System {
        System {
            root: Rc::new(RefCell::new(Realdir::new())),
        }
    }

    fn wd(&'a mut self) -> WD<'a> {
        let node = self.root.clone();
        WD { sys: self, node }
    }
}

struct WD<'a> {
    sys: &'a mut System,
    node: Rc<RefCell<dyn Folder + 'a>>,
}

impl<'a> WD<'a> {
    fn lookup(&mut self, path: &Path) -> Rc<RefCell<dyn Folder + 'a>> {
        let mut node = self.node.clone();
        for name in path.components() {
            let s: &OsStr = name.as_ref();
            let n = s.to_str().unwrap();
            eprintln!("{}", n);
            let tmp = node.borrow_mut().subdir(self.sys, n).unwrap();
            node = tmp;
        }
        node
    }
}

fn main() {
    let mut s = System::new();
    s.wd().lookup(Path::new("a/b/c"));
}
