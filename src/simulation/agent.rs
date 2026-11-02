use super::agent_attributes::{AgentAttributes, AgentMode};
use crate::house_properties::{house::House, position::Position};
use rand::{RngExt, SeedableRng, rngs::StdRng};

pub(crate) struct Agent {
    pub(crate) attributes: AgentAttributes,
    pub(crate) path: Vec<Position>,
    pub(crate) collected: Vec<usize>,
    rng: StdRng,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn systematic_covers_rectangle_once() {
        let mut agent = Agent::new(AgentMode::SystematicExplorationMode, 100);
        agent.explore(3, 2, &[], 10);
        let coordinates: Vec<_> = agent.path.iter().map(Position::get_position).collect();
        assert_eq!(
            coordinates,
            vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 1), (1, 0)]
        );
        assert_eq!(agent.attributes.get_agent_energy(), 95);
    }

    #[test]
    fn random_obeys_bounds_budget_and_unit_movement() {
        let mut agent = Agent::new(AgentMode::RandomExplorationMode, 500);
        agent.explore(4, 3, &[], 10);
        assert_eq!(agent.path.len(), 501);
        for pair in agent.path.windows(2) {
            assert_eq!(
                pair[0].x.abs_diff(pair[1].x) + pair[0].y.abs_diff(pair[1].y),
                1
            );
            assert!(pair[1].x < 4 && pair[1].y < 3);
        }
    }

    #[test]
    fn smart_targets_nearest_and_respects_collection_limit() {
        use crate::house_properties::{
            environment_features::EnvironmentFeatures, house_details::HouseDetails,
        };
        let houses: Vec<_> = [Position::new(2, 2), Position::new(0, 1)]
            .iter()
            .map(|&p| {
                House::new(
                    HouseDetails::generate_random_house_details(p, 0.0),
                    EnvironmentFeatures::new(),
                )
            })
            .collect();
        let mut agent = Agent::new(AgentMode::SmartExplorationMode, 20);
        agent.explore(3, 3, &houses, 1);
        assert_eq!(agent.path, vec![Position::new(0, 0), Position::new(0, 1)]);
        assert_eq!(agent.collected, vec![1]);
    }

    #[test]
    fn starting_house_is_free_and_revisits_are_not_samples() {
        use crate::house_properties::{
            environment_features::EnvironmentFeatures, house_details::HouseDetails,
        };
        let houses = vec![House::new(
            HouseDetails::generate_random_house_details(Position::new(0, 0), 0.0),
            EnvironmentFeatures::new(),
        )];
        let mut agent = Agent::new(AgentMode::RandomExplorationMode, 10);
        agent.explore(2, 1, &houses, 10);
        assert_eq!(agent.attributes.houses.len(), 1);
        assert_eq!(agent.path.len(), 11);
        let mut zero = Agent::new(AgentMode::SmartExplorationMode, 0);
        zero.explore(1, 1, &houses, 0);
        assert!(zero.collected.is_empty());
        assert_eq!(zero.path.len(), 1);
        zero.explore(1, 1, &houses, 10);
        assert_eq!(zero.collected, vec![0]);
        assert_eq!(zero.path.len(), 1);
        let mut single = Agent::new(AgentMode::RandomExplorationMode, 10);
        single.explore(1, 1, &houses, 10);
        assert_eq!(single.path.len(), 1);
        assert_eq!(single.attributes.get_agent_energy(), 10);
    }
}

impl Agent {
    pub(crate) fn new(agent_mode: AgentMode, agent_energy: u32) -> Self {
        Self {
            attributes: AgentAttributes::new(agent_mode, agent_energy),
            path: vec![Position::new(0, 0)],
            collected: Vec::new(),
            rng: StdRng::seed_from_u64(42),
        }
    }

    pub(crate) fn explore(&mut self, width: usize, height: usize, houses: &[House], limit: usize) {
        assert!(width > 0 && height > 0);
        self.observe(houses, limit);
        while self.attributes.get_agent_energy() > 0 && self.collected.len() < limit {
            let p = *self.path.last().unwrap();
            let next = match self.attributes.get_agent_mode() {
                AgentMode::RandomExplorationMode => {
                    let mut neighbours = Vec::new();
                    if p.x > 0 {
                        neighbours.push(Position::new(p.y, p.x - 1));
                    }
                    if p.x + 1 < width {
                        neighbours.push(Position::new(p.y, p.x + 1));
                    }
                    if p.y > 0 {
                        neighbours.push(Position::new(p.y - 1, p.x));
                    }
                    if p.y + 1 < height {
                        neighbours.push(Position::new(p.y + 1, p.x));
                    }
                    if neighbours.is_empty() {
                        break;
                    }
                    neighbours[self.rng.random_range(0..neighbours.len())]
                }
                AgentMode::SystematicExplorationMode => {
                    if p.y % 2 == 0 && p.x + 1 < width {
                        Position::new(p.y, p.x + 1)
                    } else if p.y % 2 == 1 && p.x > 0 {
                        Position::new(p.y, p.x - 1)
                    } else if p.y + 1 < height {
                        Position::new(p.y + 1, p.x)
                    } else {
                        break;
                    }
                }
                AgentMode::SmartExplorationMode => {
                    let target = houses
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| !self.collected.contains(i))
                        .min_by_key(|(_, h)| {
                            let q = h.house_details.get_house_position();
                            p.x.abs_diff(q.x) + p.y.abs_diff(q.y)
                        });
                    let Some((_, house)) = target else {
                        break;
                    };
                    let q = house.house_details.get_house_position();
                    if p.x != q.x {
                        Position::new(p.y, if p.x < q.x { p.x + 1 } else { p.x - 1 })
                    } else {
                        Position::new(if p.y < q.y { p.y + 1 } else { p.y - 1 }, p.x)
                    }
                }
            };
            self.path.push(next);
            self.attributes.decrement_agent_energy();
            self.observe(houses, limit);
        }
    }

    fn observe(&mut self, houses: &[House], limit: usize) {
        for (i, house) in houses.iter().enumerate() {
            if self.collected.len() >= limit {
                break;
            }
            if house.house_details.get_house_position() == *self.path.last().unwrap()
                && !self.collected.contains(&i)
            {
                self.collected.push(i);
                self.attributes.houses.push(house.clone());
            }
        }
    }
}
