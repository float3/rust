pub(crate) struct ABCToken {
    protom21object: ProtoM21Object,
}

impl ABCToken {
    pub(crate) fn new() -> ABCToken {
        ABCToken {
            protom21object: ProtoM21Object::new(),
        }
    }

    pub(crate) fn new(src: String) {
        todo!()
    }
    pub(crate) fn _reprInternal(&self) {
        todo!()
    }
    pub(crate) fn stripComment(&self, strSrc: String) -> String {
        todo!()
    }
    pub(crate) fn preParse(&self) {
        todo!()
    }
    pub(crate) fn parse(&self) {
        todo!()
    }
}
