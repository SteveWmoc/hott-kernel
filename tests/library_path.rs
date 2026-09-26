use hott_kernel::{
    AuditDeclarationKind, KernelFeature, check_and_extract_audit, compile_and_check_surface,
    parse_canonical,
};

#[test]
fn path_inverse_u0_checks_without_postulates_or_extensions() {
    let source = include_bytes!("../library/path-u0.surface");
    let core = compile_and_check_surface(source).expect("path library must elaborate and check");
    let mut module = parse_canonical(&core).expect("checked Surface output must be canonical Core");
    let audit = check_and_extract_audit(&mut module).expect("checked path library must audit");

    let declarations = audit.declarations();
    assert_eq!(declarations.len(), 2);

    let inverse = &declarations[0];
    assert_eq!(inverse.display_name(), "path_inverse");
    assert_eq!(inverse.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        inverse.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(inverse.direct().extensions().is_empty());
    assert!(inverse.direct().postulates().is_empty());
    assert!(inverse.direct().declarations().is_empty());
    assert_eq!(
        inverse.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(inverse.transitive().extensions().is_empty());
    assert!(inverse.transitive().postulates().is_empty());
    assert!(inverse.transitive().declarations().is_empty());

    let inverse_refl = &declarations[1];
    assert_eq!(inverse_refl.display_name(), "path_inverse_refl");
    assert_eq!(inverse_refl.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        inverse_refl.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(inverse_refl.direct().extensions().is_empty());
    assert!(inverse_refl.direct().postulates().is_empty());
    assert_eq!(inverse_refl.direct().declarations(), &[0]);
    assert_eq!(
        inverse_refl.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(inverse_refl.transitive().extensions().is_empty());
    assert_eq!(inverse_refl.transitive().declarations(), &[0]);
    assert!(inverse_refl.transitive().postulates().is_empty());
}
