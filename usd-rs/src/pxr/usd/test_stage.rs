//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::pxr::usd::stage::*;

    #[test]
    fn test_stage() {
        let stage = Stage::create_in_memory(InMemoryDescriptor::default());
        stage.export();
    }
}