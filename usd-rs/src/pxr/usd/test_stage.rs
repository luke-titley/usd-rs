//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::pxr::usd::stage::*;

    #[test]
    fn test_new() {
        /*
        let stage = Stage::create_new(Descriptor::from("test.usda"));
        stage.save();
        */
    }

    #[test]
    fn test_in_memory() {
        let stage = Stage::create_in_memory(InMemoryDescriptor::default());
        stage.save();
    }
}
