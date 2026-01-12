use super::tweaks::defaults::RECOMMENDED_TWEAKS;
use std::collections::HashMap;

/// GUI view mode
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ViewMode
{
        #[default]
        Tweaks,
        ConfirmApply,

        ConfirmUnsetAll,
        ConfirmRestorePoint,
        ConfirmLoadDefaults,
        SelectedTweaks,
}

/// Selection state for tree navigation
#[derive(Clone, Default, Debug)]
pub struct SelectionState
{
        pub selected_category: Option<String>,
        pub selected_tweak: Option<String>,
        pub expanded_categories: HashMap<String, bool>,
}

/// Checkbox states for all tweaks
#[derive(Debug, Clone)]
pub struct TweakStates
{
        pub states: HashMap<String, bool>,
        pub input_values: HashMap<String, String>,
}

impl Default for TweakStates
{
        fn default() -> Self
        {
                let mut states = HashMap::new();
                for &id in RECOMMENDED_TWEAKS {
                        states.insert(id.to_string(), true);
                }
                Self {
                        states,
                        input_values: HashMap::new(),
                }
        }
}

/// Snapshot of navigation state for history
#[derive(Clone, PartialEq, Debug)]
pub struct NavigationEntry
{
        pub mode: ViewMode,
        pub selected_category: Option<String>,
        pub selected_tweak: Option<String>,
        pub search_query: String,
}

#[cfg(test)]
mod tests
{
        use super::*;

        #[test]
        fn test_tweak_states_default()
        {
                let states = TweakStates::default();
                assert!(!states.states.is_empty(), "Default states should not be empty");

                // Sanity check: ensure recommended tweaks are actually enabled
                for &id in RECOMMENDED_TWEAKS {
                        assert!(
                                states.states.get(id).copied().unwrap_or(false),
                                "Recommended tweak {} should be enabled by default",
                                id
                        );
                }

                assert!(states.input_values.is_empty(), "Default input values should be empty");
        }

        #[test]
        fn test_selection_state_default()
        {
                let sel = SelectionState::default();
                assert!(sel.selected_category.is_none());
                assert!(sel.selected_tweak.is_none());
                assert!(sel.expanded_categories.is_empty());
        }
}
