use hott_kernel::{
    AuditDeclarationKind, KernelFeature, check_and_extract_audit, compile_and_check_surface,
    parse_canonical,
};

#[test]
fn path_algebra_u0_checks_without_postulates_or_extensions() {
    let source = include_bytes!("../library/path-u0.surface");
    let core = compile_and_check_surface(source).expect("path library must elaborate and check");
    let mut module = parse_canonical(&core).expect("checked Surface output must be canonical Core");
    let audit = check_and_extract_audit(&mut module).expect("checked path library must audit");

    let declarations = audit.declarations();
    assert_eq!(declarations.len(), 11);

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

    let concat = &declarations[2];
    assert_eq!(concat.display_name(), "path_concat");
    assert_eq!(concat.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        concat.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat.direct().extensions().is_empty());
    assert!(concat.direct().postulates().is_empty());
    assert!(concat.direct().declarations().is_empty());
    assert_eq!(
        concat.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat.transitive().extensions().is_empty());
    assert!(concat.transitive().postulates().is_empty());
    assert!(concat.transitive().declarations().is_empty());

    let concat_right_refl = &declarations[3];
    assert_eq!(concat_right_refl.display_name(), "path_concat_right_refl");
    assert_eq!(concat_right_refl.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        concat_right_refl.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat_right_refl.direct().extensions().is_empty());
    assert!(concat_right_refl.direct().postulates().is_empty());
    assert_eq!(concat_right_refl.direct().declarations(), &[2]);
    assert_eq!(
        concat_right_refl.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat_right_refl.transitive().extensions().is_empty());
    assert!(concat_right_refl.transitive().postulates().is_empty());
    assert_eq!(concat_right_refl.transitive().declarations(), &[2]);

    let concat_left_refl = &declarations[4];
    assert_eq!(concat_left_refl.display_name(), "path_concat_left_refl");
    assert_eq!(concat_left_refl.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        concat_left_refl.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat_left_refl.direct().extensions().is_empty());
    assert!(concat_left_refl.direct().postulates().is_empty());
    assert_eq!(concat_left_refl.direct().declarations(), &[2]);
    assert_eq!(
        concat_left_refl.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat_left_refl.transitive().extensions().is_empty());
    assert!(concat_left_refl.transitive().postulates().is_empty());
    assert_eq!(concat_left_refl.transitive().declarations(), &[2]);

    let concat_assoc = &declarations[5];
    assert_eq!(concat_assoc.display_name(), "path_concat_assoc");
    assert_eq!(concat_assoc.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        concat_assoc.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat_assoc.direct().extensions().is_empty());
    assert!(concat_assoc.direct().postulates().is_empty());
    assert_eq!(concat_assoc.direct().declarations(), &[2]);
    assert_eq!(
        concat_assoc.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(concat_assoc.transitive().extensions().is_empty());
    assert!(concat_assoc.transitive().postulates().is_empty());
    assert_eq!(concat_assoc.transitive().declarations(), &[2]);

    let transport = &declarations[6];
    assert_eq!(transport.display_name(), "transport");
    assert_eq!(transport.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        transport.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(transport.direct().extensions().is_empty());
    assert!(transport.direct().postulates().is_empty());
    assert!(transport.direct().declarations().is_empty());
    assert_eq!(
        transport.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(transport.transitive().extensions().is_empty());
    assert!(transport.transitive().postulates().is_empty());
    assert!(transport.transitive().declarations().is_empty());

    let transport_refl = &declarations[7];
    assert_eq!(transport_refl.display_name(), "transport_refl");
    assert_eq!(transport_refl.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        transport_refl.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(transport_refl.direct().extensions().is_empty());
    assert!(transport_refl.direct().postulates().is_empty());
    assert_eq!(transport_refl.direct().declarations(), &[6]);
    assert_eq!(
        transport_refl.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(transport_refl.transitive().extensions().is_empty());
    assert!(transport_refl.transitive().postulates().is_empty());
    assert_eq!(transport_refl.transitive().declarations(), &[6]);

    let ap = &declarations[8];
    assert_eq!(ap.display_name(), "ap");
    assert_eq!(ap.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        ap.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap.direct().extensions().is_empty());
    assert!(ap.direct().postulates().is_empty());
    assert!(ap.direct().declarations().is_empty());
    assert_eq!(
        ap.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap.transitive().extensions().is_empty());
    assert!(ap.transitive().postulates().is_empty());
    assert!(ap.transitive().declarations().is_empty());

    let ap_refl = &declarations[9];
    assert_eq!(ap_refl.display_name(), "ap_refl");
    assert_eq!(ap_refl.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        ap_refl.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap_refl.direct().extensions().is_empty());
    assert!(ap_refl.direct().postulates().is_empty());
    assert_eq!(ap_refl.direct().declarations(), &[8]);
    assert_eq!(
        ap_refl.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap_refl.transitive().extensions().is_empty());
    assert!(ap_refl.transitive().postulates().is_empty());
    assert_eq!(ap_refl.transitive().declarations(), &[8]);

    let ap_concat = &declarations[10];
    assert_eq!(ap_concat.display_name(), "ap_concat");
    assert_eq!(ap_concat.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        ap_concat.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap_concat.direct().extensions().is_empty());
    assert!(ap_concat.direct().postulates().is_empty());
    assert_eq!(ap_concat.direct().declarations(), &[2, 8]);
    assert_eq!(
        ap_concat.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap_concat.transitive().extensions().is_empty());
    assert!(ap_concat.transitive().postulates().is_empty());
    assert_eq!(ap_concat.transitive().declarations(), &[2, 8]);
}
