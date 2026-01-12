//! Integration tests for tweak definitions consistency.

use std::collections::HashSet;
use w11boost::gui::tweaks::{CATEGORIES, get_all_tweaks};

#[test]
fn test_tweak_integrity()
{
        let all_tweaks = get_all_tweaks();

        let mut seen_ids = HashSet::new();

        for tweak in all_tweaks {
                // 1. Check ID Uniqueness
                assert!(seen_ids.insert(tweak.id), "Duplicate tweak ID found: {}", tweak.id);

                // 2. Check basic metadata
                assert!(!tweak.name.is_empty(), "Tweak {} has empty name", tweak.id);
                assert!(
                        !tweak.description.is_empty(),
                        "Tweak {} has empty description",
                        tweak.id
                );

                // 3. Check Category validity
                assert!(
                        CATEGORIES.iter().any(|c| c.id == tweak.category),
                        "Tweak {} has invalid category: {}",
                        tweak.id,
                        tweak.category
                );

                // 4. Check Registry Operations
                for op in tweak.enabled_ops {
                        // subkey can't be empty if it's a registry Op (though implementation might vary,
                        // usually we expect a path).
                        // Based on `tweak!` macro, op structure might differ,
                        // but let's assume we can check subkey if it's exposed.
                        // If they are not pub fields, we might just rely on compilation check.
                        assert!(!op.subkey.is_empty(), "Tweak {} has reg op with empty subkey", tweak.id);
                }
        }
}
