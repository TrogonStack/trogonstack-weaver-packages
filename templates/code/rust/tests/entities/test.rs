use generated_semconv::{myappattr, myappentity, SCHEMA_URL};

#[test]
fn entity_resource_keeps_identity_description_and_schema() {
    let entity = myappentity::HostEntity::new(
        myappattr::HostNameAttr::new("worker-1"),
        [myappentity::HostEntityAttr::HostType(
            myappattr::HostTypeAttr::Virtual,
        )],
    );
    let attributes = entity.attributes();
    assert_eq!(attributes.len(), 2);
    assert_eq!(attributes[0].key.as_str(), "myapp.host.name");
    assert_eq!(attributes[0].value.as_str(), "worker-1");
    assert_eq!(attributes[1].value.as_str(), "virtual");
    let resource = entity.resource();
    assert_eq!(resource.schema_url(), Some(SCHEMA_URL));
    assert_eq!(resource.len(), 2);
    let mut copy = entity.attributes();
    copy.clear();
    assert_eq!(entity.attributes().len(), 2);
}

#[test]
fn refinement_requires_its_description_attribute() {
    let entity = myappentity::HostWorkerEntity::new(
        myappattr::HostNameAttr::new("worker-2"),
        myappattr::HostRoleAttr::new("worker"),
        [],
    );
    assert_eq!(entity.attributes().len(), 2);
    assert_eq!(entity.attributes()[1].key.as_str(), "myapp.host.role");
    assert_eq!(myappentity::HostEntity::default().resource().len(), 0);
}
