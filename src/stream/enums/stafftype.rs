pub struct StaffType {
    enum::enum: enum::Enum,
    REGULAR: ,
    OSSIA: ,
    CUE: ,
    EDITORIAL: ,
    ALTERNATE: ,
    OTHER: ,
}

impl StaffType {
    pub fn new() -> StaffType {
        StaffType {
            enum::enum: enum::Enum::new(),
            REGULAR: todo!(),
            OSSIA: todo!(),
            CUE: todo!(),
            EDITORIAL: todo!(),
            ALTERNATE: todo!(),
            OTHER: todo!(),
        }
    }
    
}