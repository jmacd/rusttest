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
    fn subdir<'a: 'b, 'b>(&'_ mut self, sys: &'a mut System, name: &str) -> WD<'b>;
}

#[derive(Debug)]
struct Realdir {
    subdirs: BTreeMap<String, Rc<RefCell<dyn Folder>>>,
}

#[derive(Debug)]
struct Dynadir {}

impl Folder for Realdir {
    fn subdir<'a: 'b, 'b>(&'_ mut self, sys: &'a mut System, name: &str) -> WD<'b> {
        let child = self
            .subdirs
            .entry(name.to_string())
            .or_insert_with(|| {
                if name.to_string() == "dynamic" {
                    Rc::new(RefCell::new(Dynadir::new()))
                } else {
                    Rc::new(RefCell::new(Realdir::new()))
                }
            })
            .clone();
        WD {
            sys: sys,
            node: child,
        }
    }
}

impl Folder for Dynadir {
    fn subdir<'a: 'b, 'b>(&'_ mut self, sys: &'a mut System, _name: &str) -> WD<'b> {
        sys.wd().lookup(Path::new("d/e/f")).unwrap()
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
    // I've also tried
    // fn lookup(&'_ mut self, path: &'_ Path) -> Option<WD<'a>> {

    fn lookup<'b>(&'b mut self, path: &'_ Path) -> Option<WD<'a>>
    where
        'a: 'b,
    {
        let node = self.node.clone();
        let mut comp = path.components();
        let first = comp.next().unwrap();
        let second = comp.as_path();

        let fosstr: &OsStr = first.as_ref();
        let fstr = fosstr.to_str().unwrap();

        let mut child = node.borrow_mut().subdir(self.sys, fstr);

        if second.as_os_str().is_empty() {
            Some(child)
        } else {
            child.lookup(second)
        }
    }
}

fn main() {
    let mut s = System::new();
    s.wd().lookup(Path::new("a/b/c"));
}
