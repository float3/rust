pub struct GlobalReferenceLine {
    humdrumline: HumdrumLine,
    position: ,
    contents: ,
    isGlobal: bool,
    isReference: bool,
    isComment: bool,
    isSpineLine: bool,
    numSpines: ,
}

impl GlobalReferenceLine {
    pub fn new() -> GlobalReferenceLine {
        GlobalReferenceLine {
            humdrumline: HumdrumLine::new(),
            position: todo!(),
            contents: todo!(),
            isGlobal: todo!(),
            isReference: todo!(),
            isComment: todo!(),
            isSpineLine: todo!(),
            numSpines: todo!(),
        }
    }
    
    pub fn new(&self, position: i32, contents: String) {
        todo!()
    }
}