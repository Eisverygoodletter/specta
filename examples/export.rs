use specta::{
    export,
    ts::{BigIntExportBehavior, ExportConfiguration},
    Type,
};

#[derive(Type)]
pub struct TypeOne {
    pub field1: String,
    pub field2: TypeTwo,
}

#[derive(Type)]
pub struct TypeTwo {
    pub my_field: String,
}

#[derive(Type)]
pub struct TypeThree {
    pub entity_id: bevy_ecs::entity::Entity,
}

fn main() {
    // This will automatically discover all types in your project!
    export::ts_with_cfg(
        "./bindings.ts",
        &ExportConfiguration::default().bigint(BigIntExportBehavior::Number),
    )
    .unwrap();

    // You can also override the export configuration.
    export::ts_with_cfg(
        "./bindings2.ts",
        // Be aware this won't be typesafe unless your using a ser/deserializer that converts BigInt types to a number.
        &ExportConfiguration::default().bigint(BigIntExportBehavior::Number),
    )
    .unwrap();
}
