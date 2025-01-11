pub(crate) struct SearchLanguage {
    common::enums::strenum: common::enums::StrEnum,
    ALL: ,
    ENGLISH: ,
    FRENCH: ,
    GERMAN: ,
    ITALIAN: ,
    RUSSIAN: ,
    SPANISH: ,
    ABBREVIATION: ,
}

impl SearchLanguage {
    pub(crate) fn new() -> SearchLanguage {
        SearchLanguage {
            common::enums::strenum: common::enums::StrEnum::new(),
            ALL: todo!(),
            ENGLISH: todo!(),
            FRENCH: todo!(),
            GERMAN: todo!(),
            ITALIAN: todo!(),
            RUSSIAN: todo!(),
            SPANISH: todo!(),
            ABBREVIATION: todo!(),
        }
    }
    
}