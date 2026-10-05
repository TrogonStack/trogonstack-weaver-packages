use generated_semconv::{myappattr, myappentity, SCHEMA_URL};
use opentelemetry::{Key, Value};
use opentelemetry_sdk::Resource;

#[test]
fn entity_exports_identity_description_and_schema() {
    let entity = myappentity::HostEntity::new(
        myappattr::HostNameAttr::from("worker-1"),
        [myappentity::HostEntityAttr::HostType(
            myappattr::HostTypeAttr::Virtual,
        )],
    );
    let attributes: Vec<_> = entity.attributes().collect();
    assert_eq!(attributes.len(), 2);
    assert_eq!(attributes[0].key.as_str(), "myapp.host.name");
    assert_eq!(attributes[0].value.as_str(), "worker-1");
    assert_eq!(attributes[1].value.as_str(), "virtual");
    assert_eq!(entity.attributes().count(), 2);
    let resource = Resource::from(entity);
    assert_eq!(resource.schema_url(), Some(SCHEMA_URL));
    assert_eq!(resource.len(), 2);
}

#[test]
fn refinement_requires_its_description_attribute() {
    let entity = myappentity::HostWorkerEntity::new(
        myappattr::HostNameAttr::from("worker-2"),
        myappattr::HostRoleAttr::from("worker"),
        [],
    );
    assert_eq!(entity.attributes().count(), 2);
    assert_eq!(
        entity.attributes().nth(1).unwrap().key.as_str(),
        "myapp.host.role"
    );
    assert_eq!(Resource::from(entity).len(), 2);
}

#[test]
fn consuming_export_preserves_owned_string_storage() {
    let mut owned = String::from("dynamic-worker");
    owned.shrink_to_fit();
    let original = owned.as_ptr();
    let entity = myappentity::HostEntity::new(myappattr::HostNameAttr::from(owned), []);
    let resource: Resource = entity.into();
    let value = resource
        .iter()
        .find(|(key, _)| **key == Key::from("myapp.host.name"))
        .unwrap()
        .1;
    let Value::String(text) = value else {
        panic!("expected string");
    };
    assert_eq!(text.as_str().as_ptr(), original);
}

#[test]
fn default_is_available_when_no_attributes_are_required() {
    let empty = myappentity::OptionalEntity::default();
    assert_eq!(empty.attributes().count(), 0);
    let resource = Resource::from(empty);
    assert_eq!(resource.len(), 0);
    assert_eq!(resource.schema_url(), Some(SCHEMA_URL));
}

#[test]
fn optional_attribute_overflow_preserves_values_and_order() {
    let options = (0..15).map(|index| {
        myappentity::HostEntityAttr::HostType(if index == 14 {
            myappattr::HostTypeAttr::Physical
        } else {
            myappattr::HostTypeAttr::Virtual
        })
    });
    let entity = myappentity::HostEntity::new(myappattr::HostNameAttr::from("overflow"), options);
    let attributes: Vec<_> = entity.attributes().collect();
    assert_eq!(attributes.len(), 16);
    assert_eq!(
        attributes.last().unwrap().value,
        Value::String("physical".into())
    );
    let resource: Resource = entity.into();
    assert_eq!(
        resource.get(&Key::from("myapp.host.type")),
        Some(Value::String("physical".into()))
    );
}
