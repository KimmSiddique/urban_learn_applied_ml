use crate::simulation::simulation_manager::TILE_SIZE;
use crate::{
    house_properties::building_type::BuildingType,
    resource_manager::ResourceManager,
    simulation::{
        agent::Agent,
        agent_attributes::AgentMode,
        simulation_manager::{CAMERA_VIEW_HEIGHT, CAMERA_VIEW_WIDTH},
    },
};
use macroquad::prelude::*;
use std::collections::VecDeque;
pub(crate) struct AgentManager {
    pub(crate) agents: VecDeque<Agent>,
    current_agent: Option<Agent>,
}

impl AgentManager {
    pub(crate) fn new() -> Self {
        Self {
            agents: VecDeque::new(),
            current_agent: None,
        }
    }
    pub(crate) fn get_agent(&mut self) -> Option<Agent> {
        self.agents.pop_front()
    }

    pub(crate) fn delegate_agent_exploration_method(
        &mut self,
        map: &[Vec<BuildingType>],
    ) -> Option<()> {
        let mode = self.current_agent.as_ref()?.attributes.get_agent_mode();

        match mode {
            AgentMode::RandomExplorationMode => self.random_exploration_mode(map),
            AgentMode::SystematicExplorationMode => self.systematic_exploration_mode(map),
            AgentMode::SmartExplorationMode => self.smart_exploration_mode(map),
        }
    }

    pub(crate) fn random_exploration_mode(&mut self, map: &[Vec<BuildingType>]) -> Option<()> {
        unimplemented!()
    }

    pub(crate) fn smart_exploration_mode(&mut self, map: &[Vec<BuildingType>]) -> Option<()> {
        unimplemented!()
    }

    pub(crate) fn systematic_exploration_mode(&mut self, map: &[Vec<BuildingType>]) -> Option<()> {
        let agent = self.current_agent.as_mut()?;

        let map_height = map.len();
        let map_width = map.first()?.len();

        let (player_y, player_x) = agent.attributes.get_agent_position();

        let mut next_x = player_x;
        let mut next_y = player_y;

        if !agent.attributes.get_is_agent_moving_in_opposite_direction() {
            if next_x + 1 < map_width {
                next_x += 1;
            } else {
                if next_y + 1 >= map_height {
                    return None;
                }

                next_y += 1;
                agent.attributes.move_opposite_direction = true;
            }
        } else {
            if next_x > 0 {
                next_x -= 1;
            } else {
                if next_y + 1 >= map_height {
                    return None;
                }

                next_y += 1;
                agent.attributes.move_opposite_direction = false;
            }
        }

        agent.attributes.set_agent_position(next_y, next_x);

        Some(())
    }

    pub(crate) async fn render_agent(
        &mut self,
        map: &Vec<Vec<BuildingType>>,
        camera: &mut Camera2D,
        resource_manager: &ResourceManager,
    ) -> Option<()> {
        camera.target = vec2(TILE_SIZE / 2.0, TILE_SIZE / 2.0);
        let map_height = map.len();
        let map_width = map[0].len();
        // camera.zoom.y *= -1.0;

        loop {
            clear_background(DARKGRAY);

            if self.delegate_agent_exploration_method(map).is_none() {
                break Some(());
            }

            let (player_y, player_x) = self.current_agent.as_mut()?.attributes.get_agent_position();
            let centre_x = (player_x as f32 + 0.5) * TILE_SIZE;
            let centre_y = (player_y as f32 + 0.5) * TILE_SIZE;

            camera.target = vec2(centre_x, centre_y);
            set_camera(camera);

            let camera_x = camera.target.x - CAMERA_VIEW_WIDTH / 2.0;
            let camera_y = camera.target.y - CAMERA_VIEW_HEIGHT / 2.0;

            let start_cam_start_x = (camera_x / TILE_SIZE).floor().max(0.0) as usize;
            let start_cam_start_y = (camera_y / TILE_SIZE).floor().max(0.0) as usize;

            let cam_end_x = ((camera_x + CAMERA_VIEW_WIDTH) / TILE_SIZE).ceil() as usize;
            let cam_end_y = ((camera_y + CAMERA_VIEW_HEIGHT) / TILE_SIZE).ceil() as usize;

            for y in start_cam_start_y..cam_end_y.min(map_height) {
                for x in start_cam_start_x..cam_end_x.min(map_width) {
                    let texture_id = &map[y][x];

                    draw_texture_ex(
                        resource_manager.get_texture(texture_id),
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                            ..Default::default()
                        },
                    );
                }
            }

            draw_texture_ex(
                resource_manager.get_agent_texture(),
                player_x as f32 * TILE_SIZE,
                player_y as f32 * TILE_SIZE,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
            );

            next_frame().await;
        }
    }
}
