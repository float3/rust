pub struct Specifier {
    enum::intenum: enum::IntEnum,
    PERFECT: ,
    MAJOR: ,
    MINOR: ,
    AUGMENTED: ,
    DIMINISHED: ,
    DBLAUG: ,
    DBLDIM: ,
    TRPAUG: ,
    TRPDIM: ,
    QUADAUG: ,
    QUADDIM: ,
}

impl Specifier {
    pub fn new() -> Specifier {
        Specifier {
            enum::intenum: enum::IntEnum::new(),
            PERFECT: todo!(),
            MAJOR: todo!(),
            MINOR: todo!(),
            AUGMENTED: todo!(),
            DIMINISHED: todo!(),
            DBLAUG: todo!(),
            DBLDIM: todo!(),
            TRPAUG: todo!(),
            TRPDIM: todo!(),
            QUADAUG: todo!(),
            QUADDIM: todo!(),
        }
    }
    
    pub fn __str__(&self) -> String {
        todo!()
    }
    pub fn __repr__(&self) {
        todo!()
    }
    pub fn niceName(&self) {
        todo!()
    }
    pub fn inversion(&self) {
        todo!()
    }
    pub fn semitonesAbovePerfect(&self) {
        todo!()
    }
    pub fn semitonesAboveMajor(&self) {
        todo!()
    }
}