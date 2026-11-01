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
    assert_eq!(declarations.len(), 51);

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

    let ap_inverse = &declarations[11];
    assert_eq!(ap_inverse.display_name(), "ap_inverse");
    assert_eq!(ap_inverse.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        ap_inverse.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap_inverse.direct().extensions().is_empty());
    assert!(ap_inverse.direct().postulates().is_empty());
    assert_eq!(ap_inverse.direct().declarations(), &[0, 8]);
    assert_eq!(
        ap_inverse.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(ap_inverse.transitive().extensions().is_empty());
    assert!(ap_inverse.transitive().postulates().is_empty());
    assert_eq!(ap_inverse.transitive().declarations(), &[0, 8]);

    let inverse_involutive = &declarations[12];
    assert_eq!(inverse_involutive.display_name(), "path_inverse_involutive");
    assert_eq!(inverse_involutive.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        inverse_involutive.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(inverse_involutive.direct().extensions().is_empty());
    assert!(inverse_involutive.direct().postulates().is_empty());
    assert_eq!(inverse_involutive.direct().declarations(), &[0]);
    assert_eq!(
        inverse_involutive.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe,
        ]
    );
    assert!(inverse_involutive.transitive().extensions().is_empty());
    assert!(inverse_involutive.transitive().postulates().is_empty());
    assert_eq!(inverse_involutive.transitive().declarations(), &[0]);
    let inverse_law = &declarations[13];
    assert_eq!(inverse_law.display_name(), "path_concat_inverse_right");
    assert_eq!(inverse_law.kind(), AuditDeclarationKind::Transparent);
    for foundation in [inverse_law.direct(), inverse_law.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe
            ]
        );
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
        assert_eq!(foundation.declarations(), &[0, 2]);
    }

    let inverse_law = &declarations[14];
    assert_eq!(inverse_law.display_name(), "path_concat_inverse_left");
    assert_eq!(inverse_law.kind(), AuditDeclarationKind::Transparent);
    for foundation in [inverse_law.direct(), inverse_law.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe
            ]
        );
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
        assert_eq!(foundation.declarations(), &[0, 2]);
    }

    let inverse_concat = &declarations[15];
    assert_eq!(inverse_concat.display_name(), "path_inverse_concat");
    assert_eq!(inverse_concat.kind(), AuditDeclarationKind::Transparent);
    for foundation in [inverse_concat.direct(), inverse_concat.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe
            ]
        );
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
        assert_eq!(foundation.declarations(), &[0, 2, 4]);
    }

    let transport_concat = &declarations[16];
    assert_eq!(transport_concat.display_name(), "transport_concat");
    assert_eq!(transport_concat.kind(), AuditDeclarationKind::Transparent);
    for foundation in [transport_concat.direct(), transport_concat.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe
            ]
        );
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
        assert_eq!(foundation.declarations(), &[2, 6]);
    }

    for (index, name) in [
        (17, "transport_inverse_after"),
        (18, "transport_inverse_before"),
    ] {
        let inverse_transport = &declarations[index];
        assert_eq!(inverse_transport.display_name(), name);
        assert_eq!(inverse_transport.kind(), AuditDeclarationKind::Transparent);
        for foundation in [inverse_transport.direct(), inverse_transport.transitive()] {
            assert_eq!(
                foundation.kernel_features(),
                &[
                    KernelFeature::Identity,
                    KernelFeature::Pi,
                    KernelFeature::Universe
                ]
            );
            assert!(foundation.extensions().is_empty());
            assert!(foundation.postulates().is_empty());
            assert_eq!(foundation.declarations(), &[0, 6]);
        }
    }

    for (index, name, dependencies) in [
        (19, "homotopy", &[][..]),
        (20, "homotopy_refl", &[19][..]),
        (21, "homotopy_symm", &[0, 19][..]),
        (22, "homotopy_trans", &[2, 19][..]),
    ] {
        let homotopy_law = &declarations[index];
        assert_eq!(homotopy_law.display_name(), name);
        assert_eq!(homotopy_law.kind(), AuditDeclarationKind::Transparent);
        let direct_features: &[KernelFeature] = if index >= 21 {
            &[KernelFeature::Pi, KernelFeature::Universe]
        } else {
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe,
            ]
        };
        assert_eq!(homotopy_law.direct().kernel_features(), direct_features);
        assert_eq!(
            homotopy_law.transitive().kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe
            ]
        );
        for foundation in [homotopy_law.direct(), homotopy_law.transitive()] {
            assert!(foundation.extensions().is_empty());
            assert!(foundation.postulates().is_empty());
            assert_eq!(foundation.declarations(), dependencies);
        }
    }

    let declaration = &declarations[23];
    assert_eq!(declaration.display_name(), "quasi_inverse");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[19]);
    assert_eq!(declaration.transitive().declarations(), &[19]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[24];
    assert_eq!(declaration.display_name(), "quasi_inverse_intro");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[19, 23]);
    assert_eq!(declaration.transitive().declarations(), &[19, 23]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[25];
    assert_eq!(declaration.display_name(), "quasi_inverse_map");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[23]);
    assert_eq!(declaration.transitive().declarations(), &[19, 23]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[26];
    assert_eq!(declaration.display_name(), "quasi_inverse_section");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[19, 23]);
    assert_eq!(declaration.transitive().declarations(), &[19, 23]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[27];
    assert_eq!(declaration.display_name(), "quasi_inverse_retraction");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[19, 23]);
    assert_eq!(declaration.transitive().declarations(), &[19, 23]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[28];
    assert_eq!(declaration.display_name(), "identity_quasi_inverse");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[23]);
    assert_eq!(declaration.transitive().declarations(), &[19, 23]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let transport_inverse = &declarations[29];
    assert_eq!(transport_inverse.display_name(), "transport_quasi_inverse");
    assert_eq!(transport_inverse.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        transport_inverse.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        transport_inverse.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        transport_inverse.direct().declarations(),
        &[0, 6, 17, 18, 23, 24]
    );
    assert_eq!(
        transport_inverse.transitive().declarations(),
        &[0, 6, 17, 18, 19, 23, 24]
    );
    for foundation in [transport_inverse.direct(), transport_inverse.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let symmetric_inverse = &declarations[30];
    assert_eq!(symmetric_inverse.display_name(), "quasi_inverse_symm");
    assert_eq!(symmetric_inverse.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        symmetric_inverse.direct().kernel_features(),
        &[KernelFeature::Pi, KernelFeature::Universe]
    );
    assert_eq!(
        symmetric_inverse.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        symmetric_inverse.direct().declarations(),
        &[23, 24, 25, 26, 27]
    );
    assert_eq!(
        symmetric_inverse.transitive().declarations(),
        &[19, 23, 24, 25, 26, 27]
    );
    for foundation in [symmetric_inverse.direct(), symmetric_inverse.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let composed_inverse = &declarations[31];
    assert_eq!(composed_inverse.display_name(), "quasi_inverse_comp");
    assert_eq!(composed_inverse.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        composed_inverse.direct().kernel_features(),
        &[KernelFeature::Pi, KernelFeature::Universe]
    );
    assert_eq!(
        composed_inverse.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        composed_inverse.direct().declarations(),
        &[2, 8, 23, 24, 25, 26, 27]
    );
    assert_eq!(
        composed_inverse.transitive().declarations(),
        &[2, 8, 19, 23, 24, 25, 26, 27]
    );
    for foundation in [composed_inverse.direct(), composed_inverse.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let homotopic_inverse = &declarations[32];
    assert_eq!(homotopic_inverse.display_name(), "quasi_inverse_homotopy");
    assert_eq!(homotopic_inverse.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        homotopic_inverse.direct().kernel_features(),
        &[KernelFeature::Pi, KernelFeature::Universe]
    );
    assert_eq!(
        homotopic_inverse.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    for foundation in [homotopic_inverse.direct(), homotopic_inverse.transitive()] {
        assert_eq!(
            foundation.declarations(),
            &[0, 2, 8, 19, 23, 24, 25, 26, 27]
        );
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let naturality = &declarations[33];
    assert_eq!(naturality.display_name(), "homotopy_naturality");
    assert_eq!(naturality.kind(), AuditDeclarationKind::Transparent);
    for foundation in [naturality.direct(), naturality.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe
            ]
        );
        assert_eq!(foundation.declarations(), &[2, 4, 8, 19]);
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    for (index, name) in [(34, "ap_identity"), (35, "ap_comp")] {
        let action_law = &declarations[index];
        assert_eq!(action_law.display_name(), name);
        assert_eq!(action_law.kind(), AuditDeclarationKind::Transparent);
        for foundation in [action_law.direct(), action_law.transitive()] {
            assert_eq!(
                foundation.kernel_features(),
                &[
                    KernelFeature::Identity,
                    KernelFeature::Pi,
                    KernelFeature::Universe
                ]
            );
            assert_eq!(foundation.declarations(), &[8]);
            assert!(foundation.extensions().is_empty());
            assert!(foundation.postulates().is_empty());
        }
    }

    for (index, name, dependencies) in [
        (36, "path_concat_left_cancel", &[0, 2, 4][..]),
        (37, "path_concat_right_cancel", &[2][..]),
    ] {
        let cancellation = &declarations[index];
        assert_eq!(cancellation.display_name(), name);
        assert_eq!(cancellation.kind(), AuditDeclarationKind::Transparent);
        for foundation in [cancellation.direct(), cancellation.transitive()] {
            assert_eq!(
                foundation.kernel_features(),
                &[
                    KernelFeature::Identity,
                    KernelFeature::Pi,
                    KernelFeature::Universe
                ]
            );
            assert_eq!(foundation.declarations(), dependencies);
            assert!(foundation.extensions().is_empty());
            assert!(foundation.postulates().is_empty());
        }
    }

    let coherence = &declarations[38];
    assert_eq!(coherence.display_name(), "homotopy_to_identity_coherence");
    assert_eq!(coherence.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(coherence.direct().declarations(), &[2, 8, 19, 33, 34, 37]);
    assert_eq!(
        coherence.transitive().declarations(),
        &[2, 4, 8, 19, 33, 34, 37]
    );
    for foundation in [coherence.direct(), coherence.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Universe
            ]
        );
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[39];
    assert_eq!(declaration.display_name(), "half_adjoint_inverse");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[8, 19]);
    assert_eq!(declaration.transitive().declarations(), &[8, 19]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[40];
    assert_eq!(declaration.display_name(), "half_adjoint_inverse_intro");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[8, 19, 39]);
    assert_eq!(declaration.transitive().declarations(), &[8, 19, 39]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[41];
    assert_eq!(declaration.display_name(), "half_adjoint_inverse_forget");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[23, 24, 39]);
    assert_eq!(
        declaration.transitive().declarations(),
        &[8, 19, 23, 24, 39]
    );
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[42];
    assert_eq!(declaration.display_name(), "half_adjoint_inverse_coherence");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[8, 39]);
    assert_eq!(declaration.transitive().declarations(), &[8, 19, 39]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[43];
    assert_eq!(declaration.display_name(), "identity_half_adjoint_inverse");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[39]);
    assert_eq!(declaration.transitive().declarations(), &[8, 19, 39]);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }
    let declaration = &declarations[44];
    assert_eq!(
        declaration.display_name(),
        "quasi_inverse_adjoint_retraction"
    );
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[KernelFeature::Pi, KernelFeature::Universe]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.direct().declarations(),
        &[0, 2, 8, 19, 23, 25, 26, 27]
    );
    assert_eq!(
        declaration.transitive().declarations(),
        &[0, 2, 8, 19, 23, 25, 26, 27]
    );
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[45];
    assert_eq!(declaration.display_name(), "quasi_inverse_adjoint_adjust");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[KernelFeature::Pi, KernelFeature::Universe]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[23, 24, 25, 26, 44]);
    assert_eq!(
        declaration.transitive().declarations(),
        &[0, 2, 8, 19, 23, 24, 25, 26, 27, 44]
    );
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }
    let declaration = &declarations[46];
    assert_eq!(declaration.display_name(), "path_concat_inverse_left_assoc");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe
        ]
    );
    assert_eq!(declaration.direct().declarations(), &[0, 2, 4, 5, 8, 14]);
    assert_eq!(
        declaration.transitive().declarations(),
        &[0, 2, 4, 5, 8, 14]
    );
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[47];
    assert_eq!(
        declaration.display_name(),
        "quasi_inverse_adjoint_coherence"
    );
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.direct().declarations(),
        &[0, 2, 8, 19, 23, 25, 26, 27, 33, 34, 35, 38, 44, 46]
    );
    assert_eq!(
        declaration.transitive().declarations(),
        &[
            0, 2, 4, 5, 8, 14, 19, 23, 25, 26, 27, 33, 34, 35, 37, 38, 44, 46
        ]
    );
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[48];
    assert_eq!(declaration.display_name(), "quasi_inverse_to_half_adjoint");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    assert_eq!(
        declaration.direct().kernel_features(),
        &[KernelFeature::Pi, KernelFeature::Universe]
    );
    assert_eq!(
        declaration.transitive().kernel_features(),
        &[
            KernelFeature::Identity,
            KernelFeature::Pi,
            KernelFeature::Sigma,
            KernelFeature::Universe
        ]
    );
    assert_eq!(
        declaration.direct().declarations(),
        &[23, 25, 26, 39, 40, 44, 47]
    );
    assert_eq!(
        declaration.transitive().declarations(),
        &[
            0, 2, 4, 5, 8, 14, 19, 23, 25, 26, 27, 33, 34, 35, 37, 38, 39, 40, 44, 46, 47
        ]
    );
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }
    let declaration = &declarations[49];
    assert_eq!(declaration.display_name(), "sigma_path");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Sigma,
                KernelFeature::Universe
            ]
        );
        assert_eq!(foundation.declarations(), &[6, 8]);
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }

    let declaration = &declarations[50];
    assert_eq!(declaration.display_name(), "sigma_path_fst");
    assert_eq!(declaration.kind(), AuditDeclarationKind::Transparent);
    for foundation in [declaration.direct(), declaration.transitive()] {
        assert_eq!(
            foundation.kernel_features(),
            &[
                KernelFeature::Identity,
                KernelFeature::Pi,
                KernelFeature::Sigma,
                KernelFeature::Universe
            ]
        );
        assert_eq!(foundation.declarations(), &[6, 8, 49]);
        assert!(foundation.extensions().is_empty());
        assert!(foundation.postulates().is_empty());
    }
}

#[test]
fn identity_quasi_inverse_projections_compute() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent identity_inverse_map_zero
  (id nat
    (app
      (app (app (app (app (ref quasi_inverse_map) nat) nat)
        (lam point (ref point)))
        (app (ref identity_quasi_inverse) nat))
      zero)
    zero)
  (refl zero))
(transparent identity_inverse_section_zero
  (id (id nat zero zero)
    (app
      (app (app (app (app (ref quasi_inverse_section) nat) nat)
        (lam point (ref point)))
        (app (ref identity_quasi_inverse) nat))
      zero)
    (refl zero))
  (refl (refl zero)))
(transparent identity_inverse_retraction_zero
  (id (id nat zero zero)
    (app
      (app (app (app (app (ref quasi_inverse_retraction) nat) nat)
        (lam point (ref point)))
        (app (ref identity_quasi_inverse) nat))
      zero)
    (refl zero))
  (refl (refl zero)))
(transparent identity_inverse_from_constructor
  (id nat
    (app
      (app (app (app (app (ref quasi_inverse_map) nat) nat)
        (lam point (ref point)))
        (app
          (app
            (app (app (app (app (ref quasi_inverse_intro) nat) nat)
              (lam point (ref point)))
              (lam point (ref point)))
            (lam point (refl (ref point))))
          (lam point (refl (ref point)))))
      zero)
    zero)
  (refl zero))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("quasi-inverse constructor and projections must compute on identity data");
}

#[test]
fn transport_quasi_inverse_projections_and_reflexivity_compute() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent transport_package_projection_0
  (pi A
    (universe 0)
    (pi P
      (pi point (ref A) (universe 0))
      (pi x
        (ref A)
        (pi y
          (ref A)
          (pi p
            (id (ref A) (ref x) (ref y))
            (pi value
              (app (ref P) (ref y))
              (id
                (app (ref P) (ref x))
                (app
                  (app
                    (app
                      (app
                        (app (ref quasi_inverse_map) (app (ref P) (ref x)))
                        (app (ref P) (ref y)))
                      (lam value
                        (app
                          (app
                            (app
                              (app (app (app (ref transport) (ref A)) (ref P)) (ref x))
                              (ref y))
                            (ref p))
                          (ref value))))
                    (app
                      (app
                        (app
                          (app (app (ref transport_quasi_inverse) (ref A)) (ref P))
                          (ref x))
                        (ref y))
                      (ref p)))
                  (ref value))
                (app
                  (app
                    (app
                      (app (app (app (ref transport) (ref A)) (ref P)) (ref y))
                      (ref x))
                    (app
                      (app (app (app (ref path_inverse) (ref A)) (ref x)) (ref y))
                      (ref p)))
                  (ref value)))))))))
  (lam A
    (lam P
      (lam x
        (lam y
          (lam p
            (lam value
              (refl
                (app
                  (app
                    (app
                      (app (app (app (ref transport) (ref A)) (ref P)) (ref y))
                      (ref x))
                    (app
                      (app (app (app (ref path_inverse) (ref A)) (ref x)) (ref y))
                      (ref p)))
                  (ref value))))))))))

(transparent transport_package_projection_1
  (pi A
    (universe 0)
    (pi P
      (pi point (ref A) (universe 0))
      (pi x
        (ref A)
        (pi y
          (ref A)
          (pi p
            (id (ref A) (ref x) (ref y))
            (pi value
              (app (ref P) (ref x))
              (id
                (id
                  (app (ref P) (ref x))
                  (app
                    (app
                      (app
                        (app (app (app (ref transport) (ref A)) (ref P)) (ref y))
                        (ref x))
                      (app
                        (app (app (app (ref path_inverse) (ref A)) (ref x)) (ref y))
                        (ref p)))
                    (app
                      (app
                        (app
                          (app (app (app (ref transport) (ref A)) (ref P)) (ref x))
                          (ref y))
                        (ref p))
                      (ref value)))
                  (ref value))
                (app
                  (app
                    (app
                      (app
                        (app (ref quasi_inverse_section) (app (ref P) (ref x)))
                        (app (ref P) (ref y)))
                      (lam value
                        (app
                          (app
                            (app
                              (app (app (app (ref transport) (ref A)) (ref P)) (ref x))
                              (ref y))
                            (ref p))
                          (ref value))))
                    (app
                      (app
                        (app
                          (app (app (ref transport_quasi_inverse) (ref A)) (ref P))
                          (ref x))
                        (ref y))
                      (ref p)))
                  (ref value))
                (app
                  (app
                    (app
                      (app
                        (app (app (ref transport_inverse_after) (ref A)) (ref P))
                        (ref x))
                      (ref y))
                    (ref p))
                  (ref value)))))))))
  (lam A
    (lam P
      (lam x
        (lam y
          (lam p
            (lam value
              (refl
                (app
                  (app
                    (app
                      (app
                        (app (app (ref transport_inverse_after) (ref A)) (ref P))
                        (ref x))
                      (ref y))
                    (ref p))
                  (ref value))))))))))

(transparent transport_package_projection_2
  (pi A
    (universe 0)
    (pi P
      (pi point (ref A) (universe 0))
      (pi x
        (ref A)
        (pi y
          (ref A)
          (pi p
            (id (ref A) (ref x) (ref y))
            (pi value
              (app (ref P) (ref y))
              (id
                (id
                  (app (ref P) (ref y))
                  (app
                    (app
                      (app
                        (app (app (app (ref transport) (ref A)) (ref P)) (ref x))
                        (ref y))
                      (ref p))
                    (app
                      (app
                        (app
                          (app (app (app (ref transport) (ref A)) (ref P)) (ref y))
                          (ref x))
                        (app
                          (app (app (app (ref path_inverse) (ref A)) (ref x)) (ref y))
                          (ref p)))
                      (ref value)))
                  (ref value))
                (app
                  (app
                    (app
                      (app
                        (app (ref quasi_inverse_retraction) (app (ref P) (ref x)))
                        (app (ref P) (ref y)))
                      (lam value
                        (app
                          (app
                            (app
                              (app (app (app (ref transport) (ref A)) (ref P)) (ref x))
                              (ref y))
                            (ref p))
                          (ref value))))
                    (app
                      (app
                        (app
                          (app (app (ref transport_quasi_inverse) (ref A)) (ref P))
                          (ref x))
                        (ref y))
                      (ref p)))
                  (ref value))
                (app
                  (app
                    (app
                      (app
                        (app (app (ref transport_inverse_before) (ref A)) (ref P))
                        (ref x))
                      (ref y))
                    (ref p))
                  (ref value)))))))))
  (lam A
    (lam P
      (lam x
        (lam y
          (lam p
            (lam value
              (refl
                (app
                  (app
                    (app
                      (app
                        (app (app (ref transport_inverse_before) (ref A)) (ref P))
                        (ref x))
                      (ref y))
                    (ref p))
                  (ref value))))))))))

(transparent transport_package_refl_0
  (id
    nat
    (app
      (app
        (app
          (app (app (ref quasi_inverse_map) nat) nat)
          (lam value
            (app
              (app
                (app (app (app (app (ref transport) nat) (lam point nat)) zero) zero)
                (refl zero))
              (ref value))))
        (app
          (app
            (app (app (app (ref transport_quasi_inverse) nat) (lam point nat)) zero)
            zero)
          (refl zero)))
      zero)
    zero)
  (refl zero))

(transparent transport_package_refl_1
  (id
    (id nat zero zero)
    (app
      (app
        (app
          (app (app (ref quasi_inverse_section) nat) nat)
          (lam value
            (app
              (app
                (app (app (app (app (ref transport) nat) (lam point nat)) zero) zero)
                (refl zero))
              (ref value))))
        (app
          (app
            (app (app (app (ref transport_quasi_inverse) nat) (lam point nat)) zero)
            zero)
          (refl zero)))
      zero)
    (refl zero))
  (refl (refl zero)))

(transparent transport_package_refl_2
  (id
    (id nat zero zero)
    (app
      (app
        (app
          (app (app (ref quasi_inverse_retraction) nat) nat)
          (lam value
            (app
              (app
                (app (app (app (app (ref transport) nat) (lam point nat)) zero) zero)
                (refl zero))
              (ref value))))
        (app
          (app
            (app (app (app (ref transport_quasi_inverse) nat) (lam point nat)) zero)
            zero)
          (refl zero)))
      zero)
    (refl zero))
  (refl (refl zero)))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("transport package projections and reflexivity must compute");
}

#[test]
fn quasi_inverse_symmetry_projections_compute_for_arbitrary_data() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent quasi_inverse_symm_projection_0
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (pi point (ref A) (ref B))
            (app
              (app
                (app (app (ref quasi_inverse_map) (ref B)) (ref A))
                (app
                  (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                  (ref witness)))
              (app
                (app (app (app (ref quasi_inverse_symm) (ref A)) (ref B)) (ref f))
                (ref witness)))
            (ref f))))))
  (lam A (lam B (lam f (lam witness (refl (ref f)))))))

(transparent quasi_inverse_symm_projection_1
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (app
              (app
                (app (app (ref homotopy) (ref B)) (lam point (ref B)))
                (lam point
                  (app
                    (ref f)
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (ref point)))))
              (lam point (ref point)))
            (app
              (app
                (app (app (ref quasi_inverse_section) (ref B)) (ref A))
                (app
                  (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                  (ref witness)))
              (app
                (app (app (app (ref quasi_inverse_symm) (ref A)) (ref B)) (ref f))
                (ref witness)))
            (app
              (app (app (app (ref quasi_inverse_retraction) (ref A)) (ref B)) (ref f))
              (ref witness)))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (refl
            (app
              (app (app (app (ref quasi_inverse_retraction) (ref A)) (ref B)) (ref f))
              (ref witness))))))))

(transparent quasi_inverse_symm_projection_2
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (app
              (app
                (app (app (ref homotopy) (ref A)) (lam point (ref A)))
                (lam point
                  (app
                    (app
                      (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                      (ref witness))
                    (app (ref f) (ref point)))))
              (lam point (ref point)))
            (app
              (app
                (app (app (ref quasi_inverse_retraction) (ref B)) (ref A))
                (app
                  (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                  (ref witness)))
              (app
                (app (app (app (ref quasi_inverse_symm) (ref A)) (ref B)) (ref f))
                (ref witness)))
            (app
              (app (app (app (ref quasi_inverse_section) (ref A)) (ref B)) (ref f))
              (ref witness)))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (refl
            (app
              (app (app (app (ref quasi_inverse_section) (ref A)) (ref B)) (ref f))
              (ref witness))))))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("symmetry must recover the original map and exchange the original proofs");
}

#[test]
fn quasi_inverse_composition_projections_and_identity_compute() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent quasi_inverse_comp_projection_0
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi C
        (universe 0)
        (pi f
          (pi point (ref A) (ref B))
          (pi g
            (pi point (ref B) (ref C))
            (pi witness_f
              (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
              (pi witness_g
                (app (app (app (ref quasi_inverse) (ref B)) (ref C)) (ref g))
                (pi value
                  (ref C)
                  (id
                    (ref A)
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_map) (ref A)) (ref C))
                          (lam point (app (ref g) (app (ref f) (ref point)))))
                        (app
                          (app
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_comp) (ref A)) (ref B))
                                  (ref C))
                                (ref f))
                              (ref g))
                            (ref witness_f))
                          (ref witness_g)))
                      (ref value))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                          (ref f))
                        (ref witness_f))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                            (ref g))
                          (ref witness_g))
                        (ref value))))))))))))
  (lam A
    (lam B
      (lam C
        (lam f
          (lam g
            (lam witness_f
              (lam witness_g
                (lam value
                  (refl
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                          (ref f))
                        (ref witness_f))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                            (ref g))
                          (ref witness_g))
                        (ref value)))))))))))))

(transparent quasi_inverse_comp_projection_1
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi C
        (universe 0)
        (pi f
          (pi point (ref A) (ref B))
          (pi g
            (pi point (ref B) (ref C))
            (pi witness_f
              (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
              (pi witness_g
                (app (app (app (ref quasi_inverse) (ref B)) (ref C)) (ref g))
                (pi value
                  (ref A)
                  (id
                    (id
                      (ref A)
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                            (ref f))
                          (ref witness_f))
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                              (ref g))
                            (ref witness_g))
                          (app (ref g) (app (ref f) (ref value)))))
                      (ref value))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_section) (ref A)) (ref C))
                          (lam point (app (ref g) (app (ref f) (ref point)))))
                        (app
                          (app
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_comp) (ref A)) (ref B))
                                  (ref C))
                                (ref f))
                              (ref g))
                            (ref witness_f))
                          (ref witness_g)))
                      (ref value))
                    (app
                      (app
                        (app
                          (app
                            (app
                              (app (ref path_concat) (ref A))
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness_f))
                                (app
                                  (app
                                    (app
                                      (app
                                        (app (ref quasi_inverse_map) (ref B))
                                        (ref C))
                                      (ref g))
                                    (ref witness_g))
                                  (app (ref g) (app (ref f) (ref value))))))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness_f))
                              (app (ref f) (ref value))))
                          (ref value))
                        (app
                          (app
                            (app
                              (app
                                (app (app (ref ap) (ref B)) (ref A))
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness_f)))
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                    (ref g))
                                  (ref witness_g))
                                (app (ref g) (app (ref f) (ref value)))))
                            (app (ref f) (ref value)))
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_section) (ref B)) (ref C))
                                (ref g))
                              (ref witness_g))
                            (app (ref f) (ref value)))))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                            (ref f))
                          (ref witness_f))
                        (ref value))))))))))))
  (lam A
    (lam B
      (lam C
        (lam f
          (lam g
            (lam witness_f
              (lam witness_g
                (lam value
                  (refl
                    (app
                      (app
                        (app
                          (app
                            (app
                              (app (ref path_concat) (ref A))
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness_f))
                                (app
                                  (app
                                    (app
                                      (app
                                        (app (ref quasi_inverse_map) (ref B))
                                        (ref C))
                                      (ref g))
                                    (ref witness_g))
                                  (app (ref g) (app (ref f) (ref value))))))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness_f))
                              (app (ref f) (ref value))))
                          (ref value))
                        (app
                          (app
                            (app
                              (app
                                (app (app (ref ap) (ref B)) (ref A))
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness_f)))
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                    (ref g))
                                  (ref witness_g))
                                (app (ref g) (app (ref f) (ref value)))))
                            (app (ref f) (ref value)))
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_section) (ref B)) (ref C))
                                (ref g))
                              (ref witness_g))
                            (app (ref f) (ref value)))))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                            (ref f))
                          (ref witness_f))
                        (ref value)))))))))))))

(transparent quasi_inverse_comp_projection_2
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi C
        (universe 0)
        (pi f
          (pi point (ref A) (ref B))
          (pi g
            (pi point (ref B) (ref C))
            (pi witness_f
              (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
              (pi witness_g
                (app (app (app (ref quasi_inverse) (ref B)) (ref C)) (ref g))
                (pi value
                  (ref C)
                  (id
                    (id
                      (ref C)
                      (app
                        (ref g)
                        (app
                          (ref f)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness_f))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                  (ref g))
                                (ref witness_g))
                              (ref value)))))
                      (ref value))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_retraction) (ref A)) (ref C))
                          (lam point (app (ref g) (app (ref f) (ref point)))))
                        (app
                          (app
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_comp) (ref A)) (ref B))
                                  (ref C))
                                (ref f))
                              (ref g))
                            (ref witness_f))
                          (ref witness_g)))
                      (ref value))
                    (app
                      (app
                        (app
                          (app
                            (app
                              (app (ref path_concat) (ref C))
                              (app
                                (ref g)
                                (app
                                  (ref f)
                                  (app
                                    (app
                                      (app
                                        (app
                                          (app (ref quasi_inverse_map) (ref A))
                                          (ref B))
                                        (ref f))
                                      (ref witness_f))
                                    (app
                                      (app
                                        (app
                                          (app
                                            (app (ref quasi_inverse_map) (ref B))
                                            (ref C))
                                          (ref g))
                                        (ref witness_g))
                                      (ref value))))))
                            (app
                              (ref g)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                    (ref g))
                                  (ref witness_g))
                                (ref value))))
                          (ref value))
                        (app
                          (app
                            (app
                              (app (app (app (ref ap) (ref B)) (ref C)) (ref g))
                              (app
                                (ref f)
                                (app
                                  (app
                                    (app
                                      (app
                                        (app (ref quasi_inverse_map) (ref A))
                                        (ref B))
                                      (ref f))
                                    (ref witness_f))
                                  (app
                                    (app
                                      (app
                                        (app
                                          (app (ref quasi_inverse_map) (ref B))
                                          (ref C))
                                        (ref g))
                                      (ref witness_g))
                                    (ref value)))))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                  (ref g))
                                (ref witness_g))
                              (ref value)))
                          (app
                            (app
                              (app
                                (app
                                  (app (ref quasi_inverse_retraction) (ref A))
                                  (ref B))
                                (ref f))
                              (ref witness_f))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                  (ref g))
                                (ref witness_g))
                              (ref value)))))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_retraction) (ref B)) (ref C))
                            (ref g))
                          (ref witness_g))
                        (ref value))))))))))))
  (lam A
    (lam B
      (lam C
        (lam f
          (lam g
            (lam witness_f
              (lam witness_g
                (lam value
                  (refl
                    (app
                      (app
                        (app
                          (app
                            (app
                              (app (ref path_concat) (ref C))
                              (app
                                (ref g)
                                (app
                                  (ref f)
                                  (app
                                    (app
                                      (app
                                        (app
                                          (app (ref quasi_inverse_map) (ref A))
                                          (ref B))
                                        (ref f))
                                      (ref witness_f))
                                    (app
                                      (app
                                        (app
                                          (app
                                            (app (ref quasi_inverse_map) (ref B))
                                            (ref C))
                                          (ref g))
                                        (ref witness_g))
                                      (ref value))))))
                            (app
                              (ref g)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                    (ref g))
                                  (ref witness_g))
                                (ref value))))
                          (ref value))
                        (app
                          (app
                            (app
                              (app (app (app (ref ap) (ref B)) (ref C)) (ref g))
                              (app
                                (ref f)
                                (app
                                  (app
                                    (app
                                      (app
                                        (app (ref quasi_inverse_map) (ref A))
                                        (ref B))
                                      (ref f))
                                    (ref witness_f))
                                  (app
                                    (app
                                      (app
                                        (app
                                          (app (ref quasi_inverse_map) (ref B))
                                          (ref C))
                                        (ref g))
                                      (ref witness_g))
                                    (ref value)))))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                  (ref g))
                                (ref witness_g))
                              (ref value)))
                          (app
                            (app
                              (app
                                (app
                                  (app (ref quasi_inverse_retraction) (ref A))
                                  (ref B))
                                (ref f))
                              (ref witness_f))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref B)) (ref C))
                                  (ref g))
                                (ref witness_g))
                              (ref value)))))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_retraction) (ref B)) (ref C))
                            (ref g))
                          (ref witness_g))
                        (ref value)))))))))))))

(transparent quasi_inverse_comp_identity_0
  (id
    nat
    (app
      (app
        (app (app (app (ref quasi_inverse_map) nat) nat) (lam point (ref point)))
        (app
          (app
            (app
              (app
                (app (app (app (ref quasi_inverse_comp) nat) nat) nat)
                (lam point (ref point)))
              (lam point (ref point)))
            (app (ref identity_quasi_inverse) nat))
          (app (ref identity_quasi_inverse) nat)))
      zero)
    zero)
  (refl zero))

(transparent quasi_inverse_comp_identity_1
  (id
    (id nat zero zero)
    (app
      (app
        (app (app (app (ref quasi_inverse_section) nat) nat) (lam point (ref point)))
        (app
          (app
            (app
              (app
                (app (app (app (ref quasi_inverse_comp) nat) nat) nat)
                (lam point (ref point)))
              (lam point (ref point)))
            (app (ref identity_quasi_inverse) nat))
          (app (ref identity_quasi_inverse) nat)))
      zero)
    (refl zero))
  (refl (refl zero)))

(transparent quasi_inverse_comp_identity_2
  (id
    (id nat zero zero)
    (app
      (app
        (app (app (app (ref quasi_inverse_retraction) nat) nat) (lam point (ref point)))
        (app
          (app
            (app
              (app
                (app (app (app (ref quasi_inverse_comp) nat) nat) nat)
                (lam point (ref point)))
              (lam point (ref point)))
            (app (ref identity_quasi_inverse) nat))
          (app (ref identity_quasi_inverse) nat)))
      zero)
    (refl zero))
  (refl (refl zero)))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("composition projections and identity data must compute");
}

#[test]
fn quasi_inverse_homotopy_projections_and_identity_compute() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent quasi_inverse_homotopy_projection_0
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi g
          (pi point (ref A) (ref B))
          (pi homotopy_fg
            (app
              (app (app (app (ref homotopy) (ref A)) (lam point (ref B))) (ref f))
              (ref g))
            (pi witness
              (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
              (id
                (pi point (ref B) (ref A))
                (app
                  (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref g))
                  (app
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_homotopy) (ref A)) (ref B))
                          (ref f))
                        (ref g))
                      (ref homotopy_fg))
                    (ref witness)))
                (app
                  (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                  (ref witness)))))))))
  (lam A
    (lam B
      (lam f
        (lam g
          (lam homotopy_fg
            (lam witness
              (refl
                (app
                  (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                  (ref witness))))))))))

(transparent quasi_inverse_homotopy_projection_1
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi g
          (pi point (ref A) (ref B))
          (pi homotopy_fg
            (app
              (app (app (app (ref homotopy) (ref A)) (lam point (ref B))) (ref f))
              (ref g))
            (pi witness
              (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
              (pi value
                (ref A)
                (id
                  (id
                    (ref A)
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (app (ref g) (ref value)))
                    (ref value))
                  (app
                    (app
                      (app
                        (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                        (ref g))
                      (app
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_homotopy) (ref A)) (ref B))
                              (ref f))
                            (ref g))
                          (ref homotopy_fg))
                        (ref witness)))
                    (ref value))
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (ref path_concat) (ref A))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (app (ref g) (ref value))))
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (app (ref f) (ref value))))
                        (ref value))
                      (app
                        (app
                          (app
                            (app
                              (app (app (ref ap) (ref B)) (ref A))
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness)))
                            (app (ref g) (ref value)))
                          (app (ref f) (ref value)))
                        (app
                          (app
                            (app
                              (app (ref path_inverse) (ref B))
                              (app (ref f) (ref value)))
                            (app (ref g) (ref value)))
                          (app (ref homotopy_fg) (ref value)))))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (ref value)))))))))))
  (lam A
    (lam B
      (lam f
        (lam g
          (lam homotopy_fg
            (lam witness
              (lam value
                (refl
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (ref path_concat) (ref A))
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (app (ref g) (ref value))))
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (app (ref f) (ref value))))
                        (ref value))
                      (app
                        (app
                          (app
                            (app
                              (app (app (ref ap) (ref B)) (ref A))
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness)))
                            (app (ref g) (ref value)))
                          (app (ref f) (ref value)))
                        (app
                          (app
                            (app
                              (app (ref path_inverse) (ref B))
                              (app (ref f) (ref value)))
                            (app (ref g) (ref value)))
                          (app (ref homotopy_fg) (ref value)))))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (ref value))))))))))))

(transparent quasi_inverse_homotopy_projection_2
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi g
          (pi point (ref A) (ref B))
          (pi homotopy_fg
            (app
              (app (app (app (ref homotopy) (ref A)) (lam point (ref B))) (ref f))
              (ref g))
            (pi witness
              (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
              (pi value
                (ref B)
                (id
                  (id
                    (ref B)
                    (app
                      (ref g)
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                            (ref f))
                          (ref witness))
                        (ref value)))
                    (ref value))
                  (app
                    (app
                      (app
                        (app (app (ref quasi_inverse_retraction) (ref A)) (ref B))
                        (ref g))
                      (app
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_homotopy) (ref A)) (ref B))
                              (ref f))
                            (ref g))
                          (ref homotopy_fg))
                        (ref witness)))
                    (ref value))
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (ref path_concat) (ref B))
                            (app
                              (ref g)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref value))))
                          (app
                            (ref f)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (ref value))))
                        (ref value))
                      (app
                        (app
                          (app
                            (app (ref path_inverse) (ref B))
                            (app
                              (ref f)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref value))))
                          (app
                            (ref g)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (ref value))))
                        (app
                          (ref homotopy_fg)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (ref value)))))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_retraction) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (ref value)))))))))))
  (lam A
    (lam B
      (lam f
        (lam g
          (lam homotopy_fg
            (lam witness
              (lam value
                (refl
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (ref path_concat) (ref B))
                            (app
                              (ref g)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref value))))
                          (app
                            (ref f)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (ref value))))
                        (ref value))
                      (app
                        (app
                          (app
                            (app (ref path_inverse) (ref B))
                            (app
                              (ref f)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref value))))
                          (app
                            (ref g)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (ref value))))
                        (app
                          (ref homotopy_fg)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (ref value)))))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_retraction) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (ref value))))))))))))

(transparent quasi_inverse_homotopy_identity_0
  (id
    nat
    (app
      (app
        (app (app (app (ref quasi_inverse_map) nat) nat) (lam point (ref point)))
        (app
          (app
            (app
              (app
                (app (app (ref quasi_inverse_homotopy) nat) nat)
                (lam point (ref point)))
              (lam point (ref point)))
            (lam point (refl (ref point))))
          (app (ref identity_quasi_inverse) nat)))
      zero)
    zero)
  (refl zero))

(transparent quasi_inverse_homotopy_identity_1
  (id
    (id nat zero zero)
    (app
      (app
        (app (app (app (ref quasi_inverse_section) nat) nat) (lam point (ref point)))
        (app
          (app
            (app
              (app
                (app (app (ref quasi_inverse_homotopy) nat) nat)
                (lam point (ref point)))
              (lam point (ref point)))
            (lam point (refl (ref point))))
          (app (ref identity_quasi_inverse) nat)))
      zero)
    (refl zero))
  (refl (refl zero)))

(transparent quasi_inverse_homotopy_identity_2
  (id
    (id nat zero zero)
    (app
      (app
        (app (app (app (ref quasi_inverse_retraction) nat) nat) (lam point (ref point)))
        (app
          (app
            (app
              (app
                (app (app (ref quasi_inverse_homotopy) nat) nat)
                (lam point (ref point)))
              (lam point (ref point)))
            (lam point (refl (ref point))))
          (app (ref identity_quasi_inverse) nat)))
      zero)
    (refl zero))
  (refl (refl zero)))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("homotopy transfer projections and identity data must compute");
}

#[test]
fn homotopy_naturality_reflexivity_preserves_the_left_unit_proof() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent homotopy_naturality_refl_computation
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi g
          (pi point (ref A) (ref B))
          (pi homotopy_fg
            (app
              (app (app (app (ref homotopy) (ref A)) (lam point (ref B))) (ref f))
              (ref g))
            (pi x
              (ref A)
              (id
                (id
                  (id (ref B) (app (ref f) (ref x)) (app (ref g) (ref x)))
                  (app
                    (app
                      (app
                        (app
                          (app (app (ref path_concat) (ref B)) (app (ref f) (ref x)))
                          (app (ref f) (ref x)))
                        (app (ref g) (ref x)))
                      (app
                        (app
                          (app
                            (app (app (app (ref ap) (ref A)) (ref B)) (ref f))
                            (ref x))
                          (ref x))
                        (refl (ref x))))
                    (app (ref homotopy_fg) (ref x)))
                  (app
                    (app
                      (app
                        (app
                          (app (app (ref path_concat) (ref B)) (app (ref f) (ref x)))
                          (app (ref g) (ref x)))
                        (app (ref g) (ref x)))
                      (app (ref homotopy_fg) (ref x)))
                    (app
                      (app
                        (app (app (app (app (ref ap) (ref A)) (ref B)) (ref g)) (ref x))
                        (ref x))
                      (refl (ref x)))))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (app (ref homotopy_naturality) (ref A)) (ref B))
                            (ref f))
                          (ref g))
                        (ref homotopy_fg))
                      (ref x))
                    (ref x))
                  (refl (ref x)))
                (app
                  (app
                    (app
                      (app (ref path_concat_left_refl) (ref B))
                      (app (ref f) (ref x)))
                    (app (ref g) (ref x)))
                  (app (ref homotopy_fg) (ref x))))))))))
  (lam A
    (lam B
      (lam f
        (lam g
          (lam homotopy_fg
            (lam x
              (refl
                (app
                  (app
                    (app
                      (app (ref path_concat_left_refl) (ref B))
                      (app (ref f) (ref x)))
                    (app (ref g) (ref x)))
                  (app (ref homotopy_fg) (ref x)))))))))))

(transparent homotopy_naturality_identity_computation
  (id
    (id (id nat zero zero) (refl zero) (refl zero))
    (app
      (app
        (app
          (app
            (app
              (app
                (app (app (ref homotopy_naturality) nat) nat)
                (lam point (ref point)))
              (lam point (ref point)))
            (lam point (refl (ref point))))
          zero)
        zero)
      (refl zero))
    (refl (refl zero)))
  (refl (refl (refl zero))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("naturality must compute to the specified left-unit proof");
}

#[test]
fn function_action_identity_and_composition_compute_at_reflexivity() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent ap_identity_reflexivity_computation
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (id
        (id
          (id (ref A) (ref x) (ref x))
          (app
            (app
              (app
                (app (app (app (ref ap) (ref A)) (ref A)) (lam point (ref point)))
                (ref x))
              (ref x))
            (refl (ref x)))
          (refl (ref x)))
        (app (app (app (app (ref ap_identity) (ref A)) (ref x)) (ref x)) (refl (ref x)))
        (refl (refl (ref x))))))
  (lam A (lam x (refl (refl (refl (ref x)))))))

(transparent ap_comp_reflexivity_computation
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi C
        (universe 0)
        (pi f
          (pi point (ref A) (ref B))
          (pi g
            (pi point (ref B) (ref C))
            (pi x
              (ref A)
              (id
                (id
                  (id
                    (ref C)
                    (app (ref g) (app (ref f) (ref x)))
                    (app (ref g) (app (ref f) (ref x))))
                  (app
                    (app
                      (app
                        (app
                          (app (app (ref ap) (ref A)) (ref C))
                          (lam point (app (ref g) (app (ref f) (ref point)))))
                        (ref x))
                      (ref x))
                    (refl (ref x)))
                  (app
                    (app
                      (app
                        (app (app (app (ref ap) (ref B)) (ref C)) (ref g))
                        (app (ref f) (ref x)))
                      (app (ref f) (ref x)))
                    (app
                      (app
                        (app (app (app (app (ref ap) (ref A)) (ref B)) (ref f)) (ref x))
                        (ref x))
                      (refl (ref x)))))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (app (app (ref ap_comp) (ref A)) (ref B)) (ref C))
                          (ref f))
                        (ref g))
                      (ref x))
                    (ref x))
                  (refl (ref x)))
                (refl (refl (app (ref g) (app (ref f) (ref x))))))))))))
  (lam A
    (lam B
      (lam C
        (lam f (lam g (lam x (refl (refl (refl (app (ref g) (app (ref f) (ref x)))))))))))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("ap laws must compute to their specified higher reflexivity proofs");
}

#[test]
fn path_cancellation_reflexivity_preserves_the_specified_proofs() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent left_cancel_reflexivity_computation
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (pi z
        (ref A)
        (pi q
          (id (ref A) (ref x) (ref z))
          (pi s
            (id (ref A) (ref x) (ref z))
            (pi equality
              (id
                (id (ref A) (ref x) (ref z))
                (app
                  (app
                    (app
                      (app (app (app (ref path_concat) (ref A)) (ref x)) (ref x))
                      (ref z))
                    (refl (ref x)))
                  (ref q))
                (app
                  (app
                    (app
                      (app (app (app (ref path_concat) (ref A)) (ref x)) (ref x))
                      (ref z))
                    (refl (ref x)))
                  (ref s)))
              (id
                (id (id (ref A) (ref x) (ref z)) (ref q) (ref s))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (app (ref path_concat_left_cancel) (ref A)) (ref x))
                            (ref x))
                          (refl (ref x)))
                        (ref z))
                      (ref q))
                    (ref s))
                  (ref equality))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (ref path_concat) (id (ref A) (ref x) (ref z)))
                          (ref q))
                        (app
                          (app
                            (app
                              (app
                                (app (app (ref path_concat) (ref A)) (ref x))
                                (ref x))
                              (ref z))
                            (refl (ref x)))
                          (ref q)))
                      (ref s))
                    (app
                      (app
                        (app
                          (app (ref path_inverse) (id (ref A) (ref x) (ref z)))
                          (app
                            (app
                              (app
                                (app
                                  (app (app (ref path_concat) (ref A)) (ref x))
                                  (ref x))
                                (ref z))
                              (refl (ref x)))
                            (ref q)))
                        (ref q))
                      (app
                        (app
                          (app (app (ref path_concat_left_refl) (ref A)) (ref x))
                          (ref z))
                        (ref q))))
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (ref path_concat) (id (ref A) (ref x) (ref z)))
                            (app
                              (app
                                (app
                                  (app
                                    (app (app (ref path_concat) (ref A)) (ref x))
                                    (ref x))
                                  (ref z))
                                (refl (ref x)))
                              (ref q)))
                          (app
                            (app
                              (app
                                (app
                                  (app (app (ref path_concat) (ref A)) (ref x))
                                  (ref x))
                                (ref z))
                              (refl (ref x)))
                            (ref s)))
                        (ref s))
                      (ref equality))
                    (app
                      (app
                        (app (app (ref path_concat_left_refl) (ref A)) (ref x))
                        (ref z))
                      (ref s)))))))))))
  (lam A
    (lam x
      (lam z
        (lam q
          (lam s
            (lam equality
              (refl
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (ref path_concat) (id (ref A) (ref x) (ref z)))
                          (ref q))
                        (app
                          (app
                            (app
                              (app
                                (app (app (ref path_concat) (ref A)) (ref x))
                                (ref x))
                              (ref z))
                            (refl (ref x)))
                          (ref q)))
                      (ref s))
                    (app
                      (app
                        (app
                          (app (ref path_inverse) (id (ref A) (ref x) (ref z)))
                          (app
                            (app
                              (app
                                (app
                                  (app (app (ref path_concat) (ref A)) (ref x))
                                  (ref x))
                                (ref z))
                              (refl (ref x)))
                            (ref q)))
                        (ref q))
                      (app
                        (app
                          (app (app (ref path_concat_left_refl) (ref A)) (ref x))
                          (ref z))
                        (ref q))))
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (ref path_concat) (id (ref A) (ref x) (ref z)))
                            (app
                              (app
                                (app
                                  (app
                                    (app (app (ref path_concat) (ref A)) (ref x))
                                    (ref x))
                                  (ref z))
                                (refl (ref x)))
                              (ref q)))
                          (app
                            (app
                              (app
                                (app
                                  (app (app (ref path_concat) (ref A)) (ref x))
                                  (ref x))
                                (ref z))
                              (refl (ref x)))
                            (ref s)))
                        (ref s))
                      (ref equality))
                    (app
                      (app
                        (app (app (ref path_concat_left_refl) (ref A)) (ref x))
                        (ref z))
                      (ref s))))))))))))

(transparent right_cancel_reflexivity_computation
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (pi y
        (ref A)
        (pi p
          (id (ref A) (ref x) (ref y))
          (pi s
            (id (ref A) (ref x) (ref y))
            (pi equality
              (id (id (ref A) (ref x) (ref y)) (ref p) (ref s))
              (id
                (id (id (ref A) (ref x) (ref y)) (ref p) (ref s))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app
                            (app (app (ref path_concat_right_cancel) (ref A)) (ref x))
                            (ref y))
                          (ref y))
                        (ref p))
                      (ref s))
                    (refl (ref y)))
                  (ref equality))
                (ref equality))))))))
  (lam A (lam x (lam y (lam p (lam s (lam equality (refl (ref equality)))))))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("cancellation must compute on arbitrary paths and equality witnesses");
}

#[test]
fn homotopy_identity_coherence_computes_at_the_identity_homotopy() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent identity_homotopy_coherence_computation
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (id
        (id (id (ref A) (ref x) (ref x)) (refl (ref x)) (refl (ref x)))
        (app
          (app
            (app
              (app (ref homotopy_to_identity_coherence) (ref A))
              (lam point (ref point)))
            (lam point (refl (ref point))))
          (ref x))
        (refl (refl (ref x))))))
  (lam A (lam x (refl (refl (refl (ref x)))))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("identity homotopy coherence must compute to higher reflexivity");
}

#[test]
fn half_adjoint_constructor_forgetful_map_and_coherence_compute() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent half_adjoint_forget_constructor
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi g
          (pi point (ref B) (ref A))
          (pi section
            (app
              (app
                (app (app (ref homotopy) (ref A)) (lam point (ref A)))
                (lam point (app (ref g) (app (ref f) (ref point)))))
              (lam point (ref point)))
            (pi retraction
              (app
                (app
                  (app (app (ref homotopy) (ref B)) (lam point (ref B)))
                  (lam point (app (ref f) (app (ref g) (ref point)))))
                (lam point (ref point)))
              (pi coherence
                (pi point
                  (ref A)
                  (id
                    (id
                      (ref B)
                      (app (ref f) (app (ref g) (app (ref f) (ref point))))
                      (app (ref f) (ref point)))
                    (app
                      (app
                        (app
                          (app (app (app (ref ap) (ref A)) (ref B)) (ref f))
                          (app (ref g) (app (ref f) (ref point))))
                        (ref point))
                      (app (ref section) (ref point)))
                    (app (ref retraction) (app (ref f) (ref point)))))
                (id
                  (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
                  (app
                    (app
                      (app (app (ref half_adjoint_inverse_forget) (ref A)) (ref B))
                      (ref f))
                    (app
                      (app
                        (app
                          (app
                            (app
                              (app
                                (app (ref half_adjoint_inverse_intro) (ref A))
                                (ref B))
                              (ref f))
                            (ref g))
                          (ref section))
                        (ref retraction))
                      (ref coherence)))
                  (app
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_intro) (ref A)) (ref B))
                          (ref f))
                        (ref g))
                      (ref section))
                    (ref retraction))))))))))
  (lam A
    (lam B
      (lam f
        (lam g
          (lam section
            (lam retraction
              (lam coherence
                (refl
                  (app
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_intro) (ref A)) (ref B))
                          (ref f))
                        (ref g))
                      (ref section))
                    (ref retraction)))))))))))

(transparent half_adjoint_coherence_constructor
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi g
          (pi point (ref B) (ref A))
          (pi section
            (app
              (app
                (app (app (ref homotopy) (ref A)) (lam point (ref A)))
                (lam point (app (ref g) (app (ref f) (ref point)))))
              (lam point (ref point)))
            (pi retraction
              (app
                (app
                  (app (app (ref homotopy) (ref B)) (lam point (ref B)))
                  (lam point (app (ref f) (app (ref g) (ref point)))))
                (lam point (ref point)))
              (pi coherence
                (pi point
                  (ref A)
                  (id
                    (id
                      (ref B)
                      (app (ref f) (app (ref g) (app (ref f) (ref point))))
                      (app (ref f) (ref point)))
                    (app
                      (app
                        (app
                          (app (app (app (ref ap) (ref A)) (ref B)) (ref f))
                          (app (ref g) (app (ref f) (ref point))))
                        (ref point))
                      (app (ref section) (ref point)))
                    (app (ref retraction) (app (ref f) (ref point)))))
                (id
                  (pi point
                    (ref A)
                    (id
                      (id
                        (ref B)
                        (app (ref f) (app (ref g) (app (ref f) (ref point))))
                        (app (ref f) (ref point)))
                      (app
                        (app
                          (app
                            (app (app (app (ref ap) (ref A)) (ref B)) (ref f))
                            (app (ref g) (app (ref f) (ref point))))
                          (ref point))
                        (app (ref section) (ref point)))
                      (app (ref retraction) (app (ref f) (ref point)))))
                  (app
                    (app
                      (app (app (ref half_adjoint_inverse_coherence) (ref A)) (ref B))
                      (ref f))
                    (app
                      (app
                        (app
                          (app
                            (app
                              (app
                                (app (ref half_adjoint_inverse_intro) (ref A))
                                (ref B))
                              (ref f))
                            (ref g))
                          (ref section))
                        (ref retraction))
                      (ref coherence)))
                  (ref coherence)))))))))
  (lam A
    (lam B
      (lam f
        (lam g (lam section (lam retraction (lam coherence (refl (ref coherence))))))))))

(transparent half_adjoint_forget_identity
  (pi A
    (universe 0)
    (id
      (app (app (app (ref quasi_inverse) (ref A)) (ref A)) (lam point (ref point)))
      (app
        (app
          (app (app (ref half_adjoint_inverse_forget) (ref A)) (ref A))
          (lam point (ref point)))
        (app (ref identity_half_adjoint_inverse) (ref A)))
      (app (ref identity_quasi_inverse) (ref A))))
  (lam A (refl (app (ref identity_quasi_inverse) (ref A)))))

(transparent half_adjoint_identity_triangle
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (id
        (id (id (ref A) (ref x) (ref x)) (refl (ref x)) (refl (ref x)))
        (app
          (app
            (app
              (app (app (ref half_adjoint_inverse_coherence) (ref A)) (ref A))
              (lam point (ref point)))
            (app (ref identity_half_adjoint_inverse) (ref A)))
          (ref x))
        (refl (refl (ref x))))))
  (lam A (lam x (refl (refl (refl (ref x)))))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("half-adjoint data must preserve its fields and compute on identity data");
}

#[test]
fn adjoint_retraction_adjustment_formula_and_projections_compute() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent adjusted_retraction_formula
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (pi b
            (ref B)
            (id
              (id
                (ref B)
                (app
                  (ref f)
                  (app
                    (app
                      (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                      (ref witness))
                    (ref b)))
                (ref b))
              (app
                (app
                  (app
                    (app (app (ref quasi_inverse_adjoint_retraction) (ref A)) (ref B))
                    (ref f))
                  (ref witness))
                (ref b))
              (app
                (app
                  (app
                    (app
                      (app
                        (app (ref path_concat) (ref B))
                        (app
                          (ref f)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (ref b))))
                      (app
                        (ref f)
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (app
                            (ref f)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (ref b))))))
                    (ref b))
                  (app
                    (app
                      (app
                        (app (ref path_inverse) (ref B))
                        (app
                          (ref f)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (app
                              (ref f)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref b))))))
                      (app
                        (ref f)
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b))))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_retraction) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (app
                        (ref f)
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b))))))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (ref path_concat) (ref B))
                          (app
                            (ref f)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (app
                                (ref f)
                                (app
                                  (app
                                    (app
                                      (app
                                        (app (ref quasi_inverse_map) (ref A))
                                        (ref B))
                                      (ref f))
                                    (ref witness))
                                  (ref b))))))
                        (app
                          (ref f)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (ref b))))
                      (ref b))
                    (app
                      (app
                        (app
                          (app (app (app (ref ap) (ref A)) (ref B)) (ref f))
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (app
                              (ref f)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref b)))))
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b)))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                            (ref f))
                          (ref witness))
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b)))))
                  (app
                    (app
                      (app
                        (app (app (ref quasi_inverse_retraction) (ref A)) (ref B))
                        (ref f))
                      (ref witness))
                    (ref b))))))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (lam b
            (refl
              (app
                (app
                  (app
                    (app
                      (app
                        (app (ref path_concat) (ref B))
                        (app
                          (ref f)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (ref b))))
                      (app
                        (ref f)
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (app
                            (ref f)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (ref b))))))
                    (ref b))
                  (app
                    (app
                      (app
                        (app (ref path_inverse) (ref B))
                        (app
                          (ref f)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (app
                              (ref f)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref b))))))
                      (app
                        (ref f)
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b))))
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_retraction) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (app
                        (ref f)
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b))))))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (ref path_concat) (ref B))
                          (app
                            (ref f)
                            (app
                              (app
                                (app
                                  (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                  (ref f))
                                (ref witness))
                              (app
                                (ref f)
                                (app
                                  (app
                                    (app
                                      (app
                                        (app (ref quasi_inverse_map) (ref A))
                                        (ref B))
                                      (ref f))
                                    (ref witness))
                                  (ref b))))))
                        (app
                          (ref f)
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (ref b))))
                      (ref b))
                    (app
                      (app
                        (app
                          (app (app (app (ref ap) (ref A)) (ref B)) (ref f))
                          (app
                            (app
                              (app
                                (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                (ref f))
                              (ref witness))
                            (app
                              (ref f)
                              (app
                                (app
                                  (app
                                    (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                                    (ref f))
                                  (ref witness))
                                (ref b)))))
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b)))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                            (ref f))
                          (ref witness))
                        (app
                          (app
                            (app
                              (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                              (ref f))
                            (ref witness))
                          (ref b)))))
                  (app
                    (app
                      (app
                        (app (app (ref quasi_inverse_retraction) (ref A)) (ref B))
                        (ref f))
                      (ref witness))
                    (ref b)))))))))))

(transparent adjoint_adjust_preserves_map
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (pi point (ref B) (ref A))
            (app
              (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
              (app
                (app
                  (app (app (ref quasi_inverse_adjoint_adjust) (ref A)) (ref B))
                  (ref f))
                (ref witness)))
            (app
              (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
              (ref witness)))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (refl
            (app
              (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
              (ref witness))))))))

(transparent adjoint_adjust_preserves_section
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (app
              (app
                (app (app (ref homotopy) (ref A)) (lam point (ref A)))
                (lam point
                  (app
                    (app
                      (app (app (app (ref quasi_inverse_map) (ref A)) (ref B)) (ref f))
                      (ref witness))
                    (app (ref f) (ref point)))))
              (lam point (ref point)))
            (app
              (app (app (app (ref quasi_inverse_section) (ref A)) (ref B)) (ref f))
              (app
                (app
                  (app (app (ref quasi_inverse_adjoint_adjust) (ref A)) (ref B))
                  (ref f))
                (ref witness)))
            (app
              (app (app (app (ref quasi_inverse_section) (ref A)) (ref B)) (ref f))
              (ref witness)))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (refl
            (app
              (app (app (app (ref quasi_inverse_section) (ref A)) (ref B)) (ref f))
              (ref witness))))))))

(transparent adjoint_adjust_preserves_retraction
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (app
              (app
                (app (app (ref homotopy) (ref B)) (lam point (ref B)))
                (lam point
                  (app
                    (ref f)
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (ref point)))))
              (lam point (ref point)))
            (app
              (app (app (app (ref quasi_inverse_retraction) (ref A)) (ref B)) (ref f))
              (app
                (app
                  (app (app (ref quasi_inverse_adjoint_adjust) (ref A)) (ref B))
                  (ref f))
                (ref witness)))
            (app
              (app
                (app (app (ref quasi_inverse_adjoint_retraction) (ref A)) (ref B))
                (ref f))
              (ref witness)))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (refl
            (app
              (app
                (app (app (ref quasi_inverse_adjoint_retraction) (ref A)) (ref B))
                (ref f))
              (ref witness))))))))

(transparent adjoint_retraction_identity
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (id
        (id (ref A) (ref x) (ref x))
        (app
          (app
            (app
              (app (app (ref quasi_inverse_adjoint_retraction) (ref A)) (ref A))
              (lam point (ref point)))
            (app (ref identity_quasi_inverse) (ref A)))
          (ref x))
        (refl (ref x)))))
  (lam A (lam x (refl (refl (ref x))))))

(transparent adjoint_adjust_identity
  (pi A
    (universe 0)
    (id
      (app (app (app (ref quasi_inverse) (ref A)) (ref A)) (lam point (ref point)))
      (app
        (app
          (app (app (ref quasi_inverse_adjoint_adjust) (ref A)) (ref A))
          (lam point (ref point)))
        (app (ref identity_quasi_inverse) (ref A)))
      (app (ref identity_quasi_inverse) (ref A))))
  (lam A (refl (app (ref identity_quasi_inverse) (ref A)))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("adjoint retraction adjustment must preserve fields and compute by its formula");
}

#[test]
fn adjointification_forgetful_map_coherence_and_identity_compute() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent adjoint_conversion_forgets_to_adjustment
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
            (app
              (app
                (app (app (ref half_adjoint_inverse_forget) (ref A)) (ref B))
                (ref f))
              (app
                (app
                  (app (app (ref quasi_inverse_to_half_adjoint) (ref A)) (ref B))
                  (ref f))
                (ref witness)))
            (app
              (app
                (app (app (ref quasi_inverse_adjoint_adjust) (ref A)) (ref B))
                (ref f))
              (ref witness)))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (refl
            (app
              (app
                (app (app (ref quasi_inverse_adjoint_adjust) (ref A)) (ref B))
                (ref f))
              (ref witness))))))))

(transparent adjoint_conversion_projects_supplied_coherence
  (pi A
    (universe 0)
    (pi B
      (universe 0)
      (pi f
        (pi point (ref A) (ref B))
        (pi witness
          (app (app (app (ref quasi_inverse) (ref A)) (ref B)) (ref f))
          (id
            (pi a
              (ref A)
              (id
                (id
                  (ref B)
                  (app
                    (ref f)
                    (app
                      (app
                        (app
                          (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                          (ref f))
                        (ref witness))
                      (app (ref f) (ref a))))
                  (app (ref f) (ref a)))
                (app
                  (app
                    (app
                      (app (app (app (ref ap) (ref A)) (ref B)) (ref f))
                      (app
                        (app
                          (app
                            (app (app (ref quasi_inverse_map) (ref A)) (ref B))
                            (ref f))
                          (ref witness))
                        (app (ref f) (ref a))))
                    (ref a))
                  (app
                    (app
                      (app
                        (app (app (ref quasi_inverse_section) (ref A)) (ref B))
                        (ref f))
                      (ref witness))
                    (ref a)))
                (app
                  (app
                    (app
                      (app (app (ref quasi_inverse_adjoint_retraction) (ref A)) (ref B))
                      (ref f))
                    (ref witness))
                  (app (ref f) (ref a)))))
            (app
              (app
                (app (app (ref half_adjoint_inverse_coherence) (ref A)) (ref B))
                (ref f))
              (app
                (app
                  (app (app (ref quasi_inverse_to_half_adjoint) (ref A)) (ref B))
                  (ref f))
                (ref witness)))
            (app
              (app
                (app (app (ref quasi_inverse_adjoint_coherence) (ref A)) (ref B))
                (ref f))
              (ref witness)))))))
  (lam A
    (lam B
      (lam f
        (lam witness
          (refl
            (app
              (app
                (app (app (ref quasi_inverse_adjoint_coherence) (ref A)) (ref B))
                (ref f))
              (ref witness))))))))

(transparent adjoint_conversion_identity
  (pi A
    (universe 0)
    (id
      (app
        (app (app (ref half_adjoint_inverse) (ref A)) (ref A))
        (lam point (ref point)))
      (app
        (app
          (app (app (ref quasi_inverse_to_half_adjoint) (ref A)) (ref A))
          (lam point (ref point)))
        (app (ref identity_quasi_inverse) (ref A)))
      (app (ref identity_half_adjoint_inverse) (ref A))))
  (lam A (refl (app (ref identity_half_adjoint_inverse) (ref A)))))

(transparent adjoint_triangle_identity
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (id
        (id (id (ref A) (ref x) (ref x)) (refl (ref x)) (refl (ref x)))
        (app
          (app
            (app
              (app (app (ref quasi_inverse_adjoint_coherence) (ref A)) (ref A))
              (lam point (ref point)))
            (app (ref identity_quasi_inverse) (ref A)))
          (ref x))
        (refl (refl (ref x))))))
  (lam A (lam x (refl (refl (refl (ref x)))))))

(transparent inverse_concat_assoc_reflexivity
  (pi A
    (universe 0)
    (pi x
      (ref A)
      (id
        (id (id (ref A) (ref x) (ref x)) (refl (ref x)) (refl (ref x)))
        (app
          (app
            (app
              (app
                (app (app (ref path_concat_inverse_left_assoc) (ref A)) (ref x))
                (ref x))
              (ref x))
            (refl (ref x)))
          (refl (ref x)))
        (refl (refl (ref x))))))
  (lam A (lam x (refl (refl (refl (ref x)))))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("adjointification must preserve its specified fields and compute on identity data");
}

#[test]
fn dependent_pair_paths_compute_at_reflexive_components() {
    let source = concat!(
        include_str!("../library/path-u0.surface"),
        r#"
(transparent sigma_path_reflexive_base
  (pi A
    (universe 0)
    (pi P
      (pi point (ref A) (universe 0))
      (pi x
        (ref A)
        (pi u
          (app (ref P) (ref x))
          (pi v
            (app (ref P) (ref x))
            (pi q
              (id (app (ref P) (ref x)) (ref u) (ref v))
              (id
                (id
                  (sigma point (ref A) (app (ref P) (ref point)))
                  (pair (ref x) (ref u))
                  (pair (ref x) (ref v)))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (app (app (ref sigma_path) (ref A)) (ref P)) (ref x))
                          (ref x))
                        (refl (ref x)))
                      (ref u))
                    (ref v))
                  (ref q))
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (ref ap) (app (ref P) (ref x)))
                          (sigma point (ref A) (app (ref P) (ref point))))
                        (lam value (pair (ref x) (ref value))))
                      (ref u))
                    (ref v))
                  (ref q)))))))))
  (lam A
    (lam P
      (lam x
        (lam u
          (lam v
            (lam q
              (refl
                (app
                  (app
                    (app
                      (app
                        (app
                          (app (ref ap) (app (ref P) (ref x)))
                          (sigma point (ref A) (app (ref P) (ref point))))
                        (lam value (pair (ref x) (ref value))))
                      (ref u))
                    (ref v))
                  (ref q))))))))))

(transparent sigma_path_all_reflexivity
  (pi A
    (universe 0)
    (pi P
      (pi point (ref A) (universe 0))
      (pi x
        (ref A)
        (pi u
          (app (ref P) (ref x))
          (id
            (id
              (sigma point (ref A) (app (ref P) (ref point)))
              (pair (ref x) (ref u))
              (pair (ref x) (ref u)))
            (app
              (app
                (app
                  (app
                    (app
                      (app (app (app (ref sigma_path) (ref A)) (ref P)) (ref x))
                      (ref x))
                    (refl (ref x)))
                  (ref u))
                (ref u))
              (refl (ref u)))
            (refl (ann (pair (ref x) (ref u)) (sigma point (ref A) (app (ref P) (ref point))))))))))
  (lam A (lam P (lam x (lam u (refl (refl (ann (pair (ref x) (ref u)) (sigma point (ref A) (app (ref P) (ref point)))))))))))

(transparent sigma_path_fst_all_reflexivity
  (pi A
    (universe 0)
    (pi P
      (pi point (ref A) (universe 0))
      (pi x
        (ref A)
        (pi u
          (app (ref P) (ref x))
          (id
            (id (id (ref A) (ref x) (ref x)) (refl (ref x)) (refl (ref x)))
            (app
              (app
                (app
                  (app
                    (app
                      (app (app (app (ref sigma_path_fst) (ref A)) (ref P)) (ref x))
                      (ref x))
                    (refl (ref x)))
                  (ref u))
                (ref u))
              (refl (ref u)))
            (refl (refl (ref x))))))))
  (lam A (lam P (lam x (lam u (refl (refl (refl (ref x)))))))))
"#
    );
    compile_and_check_surface(source.as_bytes())
        .expect("dependent pair paths must compute by fiber inclusion and project correctly");
}
