//------------------------------------------------------------------------------
// Luke Titley : from+usd_rs@luketitley.com
//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::pxr::sdf;
    use crate::pxr::tf;
    use crate::pxr::usd::*;
    use crate::pxr::vt;

    #[test]
    fn test_new() {
        let stage = Stage::create_new(StageDescriptor::from("test.usda"));
        stage.save();
    }

    #[test]
    fn test_in_memory() {
        let stage = Stage::create_in_memory(StageInMemoryDescriptor::default());
        stage.save();
    }

    #[test]
    fn test_define_prim() {
        let stage =
            Stage::create_new(StageDescriptor::from("define_prim.usda"));
        stage.define_prim(
            &sdf::Path::from("/root/world/test"),
            &tf::Token::default(),
        );
        stage.save();
    }

    #[test]
    fn test_create_attribute() {
        let stage = Stage::create_new(StageDescriptor::from(
            "create_attribute_prim.usda",
        ));
        let prim = stage.define_prim(
            &sdf::Path::from("/root/world/test"),
            &tf::Token::default(),
        );

        prim.create_attribute(AttributeDescriptor {
            name: tf::Token::from("lukes_attr"),
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::from("int")),
        });

        stage.save();
    }

    #[test]
    fn test_set_attribute() {
        let stage =
            Stage::create_new(StageDescriptor::from("set_attribute_prim.usda"));
        let prim = stage.define_prim(
            &sdf::Path::from("/root/world/test"),
            &tf::Token::default(),
        );

        let attr = prim.create_attribute(AttributeDescriptor {
            name: tf::Token::from("lukes_attr"),
            type_name: sdf::Schema::get_instance()
                .find_type(&tf::Token::from("uint")),
        });

        attr.set(&vt::Value::from(&123u32), TimeCode::default());

        stage.save();
    }
}
