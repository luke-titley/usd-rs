//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
pub type Result<T> = std::result::Result<T, crate::pxr::Error>;

pub type NoResult = Result<()>;
