//! CVA-style Variant System for Lumen Blocks
//!
//! This module provides a class-variance-authority (CVA) inspired system for managing
//! component variants in Rust. It allows defining base classes, variant classes, and
//! compound variants that are applied based on variant selections.
//!
//! # Example
//!
//! ```rust
//! use lumen_blocks::variants::{VariantConfig, cva};
//!
//! let button = cva(
//!     "inline-flex items-center justify-center rounded-md font-medium transition-colors",
//!     &[
//!         ("variant", &[
//!             ("default", "bg-primary text-primary-foreground hover:bg-primary/90"),
//!             ("destructive", "bg-destructive text-destructive-foreground hover:bg-destructive/90"),
//!             ("outline", "border border-input bg-background hover:bg-accent hover:text-accent-foreground"),
//!             ("secondary", "bg-secondary text-secondary-foreground hover:bg-secondary/80"),
//!             ("ghost", "hover:bg-accent hover:text-accent-foreground"),
//!             ("link", "text-primary underline-offset-4 hover:underline"),
//!         ]),
//!         ("size", &[
//!             ("default", "h-10 px-4 py-2"),
//!             ("sm", "h-9 rounded-md px-3"),
//!             ("lg", "h-11 rounded-md px-8"),
//!             ("icon", "h-10 w-10"),
//!         ]),
//!     ],
//!     &[("variant", "default"), ("size", "default")],
//!     &[],
//! );
//!
//! // Get classes for specific variant combination
//! let classes = button.apply(&[("variant", "destructive"), ("size", "lg")]);
//! ```

use std::collections::HashMap;

/// A compound variant condition that applies additional classes when
/// multiple variant conditions are met.
#[derive(Clone, Debug)]
pub struct CompoundVariant<'a> {
    /// The conditions that must all be met for this compound variant to apply.
    /// Each tuple is (variant_name, variant_value).
    pub conditions: &'a [(&'a str, &'a str)],
    /// The CSS classes to apply when all conditions are met.
    pub class: &'a str,
}

impl<'a> CompoundVariant<'a> {
    /// Create a new compound variant.
    pub const fn new(conditions: &'a [(&'a str, &'a str)], class: &'a str) -> Self {
        Self { conditions, class }
    }

    /// Check if this compound variant's conditions are satisfied by the given selections.
    fn matches(&self, selections: &HashMap<&str, &str>) -> bool {
        self.conditions.iter().all(|(key, value)| {
            selections.get(key).map(|v| *v == *value).unwrap_or(false)
        })
    }
}

/// Configuration for a CVA-style variant system.
#[derive(Clone, Debug)]
pub struct VariantConfig<'a> {
    /// Base CSS classes that are always applied.
    pub base: &'a str,
    /// Variant definitions: (variant_name, [(value, classes), ...])
    pub variants: &'a [(&'a str, &'a [(&'a str, &'a str)])],
    /// Default variants to use when a variant is not specified.
    pub default_variants: &'a [(&'a str, &'a str)],
    /// Compound variants that apply when multiple conditions are met.
    pub compound_variants: &'a [CompoundVariant<'a>],
}

impl<'a> VariantConfig<'a> {
    /// Create a new variant configuration.
    pub const fn new(
        base: &'a str,
        variants: &'a [(&'a str, &'a [(&'a str, &'a str)])],
        default_variants: &'a [(&'a str, &'a str)],
        compound_variants: &'a [CompoundVariant<'a>],
    ) -> Self {
        Self {
            base,
            variants,
            default_variants,
            compound_variants,
        }
    }

    /// Apply variant selections and return the combined CSS class string.
    ///
    /// # Arguments
    /// * `selections` - Variant selections as (variant_name, variant_value) tuples.
    ///
    /// # Returns
    /// A space-separated string of CSS classes.
    pub fn apply(&self, selections: &[(&str, &str)]) -> String {
        let mut classes = Vec::with_capacity(16);

        // Always include base classes
        if !self.base.is_empty() {
            classes.push(self.base);
        }

        // Build selection map, merging with defaults
        let mut selection_map: HashMap<&str, &str> = self
            .default_variants
            .iter()
            .cloned()
            .collect();

        for (key, value) in selections {
            selection_map.insert(key, value);
        }

        // Apply variant classes
        for (variant_name, variant_options) in self.variants {
            if let Some(selected_value) = selection_map.get(variant_name) {
                if let Some((_, class)) = variant_options
                    .iter()
                    .find(|(value, _)| value == selected_value)
                {
                    if !class.is_empty() {
                        classes.push(class);
                    }
                }
            }
        }

        // Apply compound variants
        for compound in self.compound_variants {
            if compound.matches(&selection_map) && !compound.class.is_empty() {
                classes.push(compound.class);
            }
        }

        classes.join(" ")
    }

    /// Apply variant selections and append additional classes.
    ///
    /// This is useful for adding custom classes alongside variant classes.
    pub fn apply_with(&self, selections: &[(&str, &str)], additional: &str) -> String {
        let base = self.apply(selections);
        if additional.is_empty() {
            base
        } else if base.is_empty() {
            additional.to_string()
        } else {
            format!("{} {}", base, additional)
        }
    }
}

/// Convenience function to create a VariantConfig.
///
/// This is the primary way to define variants, similar to CVA's `cva()` function.
pub const fn cva<'a>(
    base: &'a str,
    variants: &'a [(&'a str, &'a [(&'a str, &'a str)])],
    default_variants: &'a [(&'a str, &'a str)],
    compound_variants: &'a [CompoundVariant<'a>],
) -> VariantConfig<'a> {
    VariantConfig::new(base, variants, default_variants, compound_variants)
}

/// Helper function to merge multiple class strings, filtering empty strings.
pub fn cn(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|c| !c.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Helper function to conditionally include a class.
pub fn class_if(condition: bool, class: &str) -> &str {
    if condition {
        class
    } else {
        ""
    }
}

/// Helper function to select between two classes based on a condition.
pub fn class_switch<'a>(condition: bool, if_true: &'a str, if_false: &'a str) -> &'a str {
    if condition {
        if_true
    } else {
        if_false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_variants() {
        let config = cva(
            "base-class",
            &[
                ("variant", &[
                    ("primary", "bg-primary"),
                    ("secondary", "bg-secondary"),
                ]),
                ("size", &[
                    ("sm", "h-8"),
                    ("md", "h-10"),
                    ("lg", "h-12"),
                ]),
            ],
            &[("variant", "primary"), ("size", "md")],
            &[],
        );

        // Test with defaults
        assert_eq!(
            config.apply(&[]),
            "base-class bg-primary h-10"
        );

        // Test with custom selections
        assert_eq!(
            config.apply(&[("variant", "secondary"), ("size", "lg")]),
            "base-class bg-secondary h-12"
        );

        // Test with partial selection (uses default for unspecified)
        assert_eq!(
            config.apply(&[("size", "sm")]),
            "base-class bg-primary h-8"
        );
    }

    #[test]
    fn test_compound_variants() {
        let config = cva(
            "base",
            &[
                ("variant", &[
                    ("outline", "border"),
                    ("solid", "bg-solid"),
                ]),
                ("size", &[
                    ("icon", "w-10 h-10"),
                    ("default", "px-4"),
                ]),
            ],
            &[("variant", "solid"), ("size", "default")],
            &[CompoundVariant::new(
                &[("variant", "outline"), ("size", "icon")],
                "border-2",
            )],
        );

        // Compound variant should not apply
        assert_eq!(
            config.apply(&[("variant", "solid"), ("size", "icon")]),
            "base bg-solid w-10 h-10"
        );

        // Compound variant should apply
        assert_eq!(
            config.apply(&[("variant", "outline"), ("size", "icon")]),
            "base border w-10 h-10 border-2"
        );
    }

    #[test]
    fn test_cn() {
        assert_eq!(cn(&["a", "b", "c"]), "a b c");
        assert_eq!(cn(&["a", "", "c"]), "a c");
        assert_eq!(cn(&["", "", ""]), "");
    }

    #[test]
    fn test_class_if() {
        assert_eq!(class_if(true, "visible"), "visible");
        assert_eq!(class_if(false, "visible"), "");
    }

    #[test]
    fn test_class_switch() {
        assert_eq!(class_switch(true, "yes", "no"), "yes");
        assert_eq!(class_switch(false, "yes", "no"), "no");
    }
}
