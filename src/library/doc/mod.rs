#[allow(non_camel_case_types)]
#[derive(Clone,Copy)]
pub enum DocType {
    NO_STRICT,
    STRICT,
    STRUCT_INIT,
    NO_RETURN,
    INCLUDE_STMT,
    NO_POINTER,
    CONSTRUCTOR,
    CRUST,
    DEFAULT,
}

impl DocType {
    pub fn get_doc(self) -> &'static str {
        
	" "
    }
}
