use crate::ops::ExpectArray;
use crate::ops::ExpectIsoDateTime;
use crate::ops::ExpectObject;
use crate::ops::ExpectString;

pub fn object() -> ExpectObject {
    ExpectObject::new()
}

pub fn string() -> ExpectString {
    ExpectString::new()
}

pub fn array() -> ExpectArray {
    ExpectArray::new()
}

pub fn iso_date_time() -> ExpectIsoDateTime {
    ExpectIsoDateTime::new()
}
