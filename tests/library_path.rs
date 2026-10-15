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
    assert_eq!(declarations.len(), 29);

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
