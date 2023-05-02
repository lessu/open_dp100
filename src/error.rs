#[derive(Debug)]
pub enum OpenDP100Error{
    DRIVER,
    DEVICE,
    DEVICE_OPERATION,
    INVALID_PARAM
}
// impl Display for OpenDP100Error{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
//     }
// }

// impl Error for OpenDP100Error{}