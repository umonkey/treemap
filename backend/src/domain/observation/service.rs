use super::models::ObservationFlags;
use super::repository::ObservationRepository;
use crate::domain::observation::Observation;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use crate::utils::{get_timestamp, get_unique_id};
use std::sync::Arc;

pub struct ObservationService {
    repository: Arc<ObservationRepository>,
}

impl ObservationService {
    pub async fn get(&self, tree_id: u64) -> Result<Observation> {
        let obs = self.repository.get_last_by_tree(tree_id).await?;
        Ok(obs.unwrap_or_else(|| Observation::empty(tree_id)))
    }

    pub async fn add(
        &self,
        tree_id: u64,
        user_id: u64,
        flags: ObservationFlags,
    ) -> Result<Observation> {
        let now = get_timestamp();

        if let Some(mut last) = self.repository.get_last_by_tree(tree_id).await? {
            if last.matches_flags(&flags) {
                return Ok(last);
            }

            // If the last observation is from the same user within 1 hour, update it.
            if last.created_by == user_id && now - last.created_at < 3600 {
                last.created_at = now;
                last.bark_damage = flags.bark_damage;
                last.dry_branches = flags.dry_branches;
                last.leaking = flags.leaking;
                last.root_damage = flags.root_damage;
                last.open_roots = flags.open_roots;
                last.topping = flags.topping;
                last.fungal_bodies = flags.fungal_bodies;
                last.vfork = flags.vfork;
                last.cavities = flags.cavities;
                last.vines = flags.vines;
                last.nests = flags.nests;
                last.nesting_boxes = flags.nesting_boxes;
                last.bug_holes = flags.bug_holes;

                self.repository.update(&last).await?;

                return Ok(last);
            }
        }

        let id = get_unique_id()?;

        let observation = Observation {
            id,
            tree_id,
            created_at: now,
            created_by: user_id,
            bark_damage: flags.bark_damage,
            dry_branches: flags.dry_branches,
            leaking: flags.leaking,
            root_damage: flags.root_damage,
            open_roots: flags.open_roots,
            topping: flags.topping,
            fungal_bodies: flags.fungal_bodies,
            vfork: flags.vfork,
            cavities: flags.cavities,
            vines: flags.vines,
            nests: flags.nests,
            nesting_boxes: flags.nesting_boxes,
            bug_holes: flags.bug_holes,
        };

        self.repository.add(&observation).await?;

        Ok(observation)
    }
}

impl Locatable for ObservationService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            repository: locator.get::<ObservationRepository>()?,
        })
    }
}
