use crate::infra::database::{Attributes, Value};
use crate::types::Result;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub id: u64,
    pub tree_id: u64,
    pub created_at: u64,
    pub created_by: u64,
    pub bark_damage: bool,
    pub dry_branches: bool,
    pub leaking: bool,
    pub root_damage: bool,
    pub open_roots: bool,
    pub topping: bool,
    pub fungal_bodies: bool,
    pub vfork: bool,
    pub cavities: bool,
    pub vines: bool,
    pub nests: bool,
    pub nesting_boxes: bool,
    pub bug_holes: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObservationFlags {
    pub bark_damage: bool,
    pub dry_branches: bool,
    pub leaking: bool,
    pub root_damage: bool,
    pub open_roots: bool,
    pub topping: bool,
    pub fungal_bodies: bool,
    pub vfork: bool,
    pub cavities: bool,
    pub vines: bool,
    pub nests: bool,
    pub nesting_boxes: bool,
    pub bug_holes: bool,
}

impl Observation {
    pub fn empty(tree_id: u64) -> Self {
        Self {
            id: 0,
            tree_id,
            created_at: 0,
            created_by: 0,
            bark_damage: false,
            dry_branches: false,
            leaking: false,
            root_damage: false,
            open_roots: false,
            topping: false,
            fungal_bodies: false,
            vfork: false,
            cavities: false,
            vines: false,
            nests: false,
            nesting_boxes: false,
            bug_holes: false,
        }
    }

    pub fn matches_flags(&self, flags: &ObservationFlags) -> bool {
        self.bark_damage == flags.bark_damage
            && self.dry_branches == flags.dry_branches
            && self.leaking == flags.leaking
            && self.root_damage == flags.root_damage
            && self.open_roots == flags.open_roots
            && self.topping == flags.topping
            && self.fungal_bodies == flags.fungal_bodies
            && self.vfork == flags.vfork
            && self.cavities == flags.cavities
            && self.vines == flags.vines
            && self.nests == flags.nests
            && self.nesting_boxes == flags.nesting_boxes
            && self.bug_holes == flags.bug_holes
    }

    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        Ok(Self {
            id: attributes.require_u64("id")?,
            tree_id: attributes.require_u64("tree_id")?,
            created_at: attributes.require_u64("created_at")?,
            created_by: attributes.require_u64("created_by")?,
            bark_damage: attributes.require_i64("bark_damage")? != 0,
            dry_branches: attributes.require_i64("dry_branches")? != 0,
            leaking: attributes.require_i64("leaking")? != 0,
            root_damage: attributes.require_i64("root_damage")? != 0,
            open_roots: attributes.require_i64("open_roots")? != 0,
            topping: attributes.require_i64("topping")? != 0,
            fungal_bodies: attributes.require_i64("fungal_bodies")? != 0,
            vfork: attributes.require_i64("vfork")? != 0,
            cavities: attributes.require_i64("cavities")? != 0,
            vines: attributes.require_i64("vines")? != 0,
            nests: attributes.require_i64("nests")? != 0,
            nesting_boxes: attributes.require_i64("nesting_boxes")? != 0,
            bug_holes: attributes.require_i64("bug_holes")? != 0,
        })
    }

    pub fn to_attributes(&self) -> Attributes {
        Attributes::from(&[
            ("id".to_string(), Value::from(self.id as i64)),
            ("tree_id".to_string(), Value::from(self.tree_id as i64)),
            (
                "created_at".to_string(),
                Value::from(self.created_at as i64),
            ),
            (
                "created_by".to_string(),
                Value::from(self.created_by as i64),
            ),
            (
                "bark_damage".to_string(),
                Value::from(if self.bark_damage { 1 } else { 0 }),
            ),
            (
                "dry_branches".to_string(),
                Value::from(if self.dry_branches { 1 } else { 0 }),
            ),
            (
                "leaking".to_string(),
                Value::from(if self.leaking { 1 } else { 0 }),
            ),
            (
                "root_damage".to_string(),
                Value::from(if self.root_damage { 1 } else { 0 }),
            ),
            (
                "open_roots".to_string(),
                Value::from(if self.open_roots { 1 } else { 0 }),
            ),
            (
                "topping".to_string(),
                Value::from(if self.topping { 1 } else { 0 }),
            ),
            (
                "fungal_bodies".to_string(),
                Value::from(if self.fungal_bodies { 1 } else { 0 }),
            ),
            (
                "vfork".to_string(),
                Value::from(if self.vfork { 1 } else { 0 }),
            ),
            (
                "cavities".to_string(),
                Value::from(if self.cavities { 1 } else { 0 }),
            ),
            (
                "vines".to_string(),
                Value::from(if self.vines { 1 } else { 0 }),
            ),
            (
                "nests".to_string(),
                Value::from(if self.nests { 1 } else { 0 }),
            ),
            (
                "nesting_boxes".to_string(),
                Value::from(if self.nesting_boxes { 1 } else { 0 }),
            ),
            (
                "bug_holes".to_string(),
                Value::from(if self.bug_holes { 1 } else { 0 }),
            ),
        ])
    }
}
