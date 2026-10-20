use crate::house_properties::building_type::BuildingType;
use macroquad::prelude::*;
use std::collections::HashMap;

pub(crate) struct ResourceManager {
    resources: HashMap<BuildingType, Texture2D>,
    agent_texture: Texture2D,
}

impl ResourceManager {
    pub(crate) async fn new() -> Self {
        let empty_texture = load_texture("assets/empty.jpg")
            .await
            .expect("Error loading empty texture");
        let house_texture = load_texture("assets/house.jpg")
            .await
            .expect("Error loading house texture");
        let school_texture = load_texture("assets/school.jpg")
            .await
            .expect("Error loading school texture");
        let shop_texture = load_texture("assets/shop.jpg")
            .await
            .expect("Error loading shop texture");
        let park_texture = load_texture("assets/park.jpg")
            .await
            .expect("Error loading park texture");
        let factory_texture = load_texture("assets/factory.jpg")
            .await
            .expect("Error loading factory texture");
        let agent_texture = load_texture("assets/agent.png")
            .await
            .expect("Error loading agent texture");

        empty_texture.set_filter(FilterMode::Nearest);
        house_texture.set_filter(FilterMode::Nearest);
        school_texture.set_filter(FilterMode::Nearest);
        shop_texture.set_filter(FilterMode::Nearest);
        park_texture.set_filter(FilterMode::Nearest);
        factory_texture.set_filter(FilterMode::Nearest);
        agent_texture.set_filter(FilterMode::Nearest);

        let textures: HashMap<BuildingType, Texture2D> = HashMap::from([
            (BuildingType::Empty, empty_texture),
            (BuildingType::House, house_texture),
            (BuildingType::School, school_texture),
            (BuildingType::Shop, shop_texture),
            (BuildingType::Park, park_texture),
            (BuildingType::Factory, factory_texture),
        ]);

        Self {
            resources: textures,
            agent_texture: agent_texture,
        }
    }

    pub(crate) fn get_texture(&self, texture_id: &BuildingType) -> &Texture2D {
        self.resources.get(texture_id).unwrap()
    }

    pub(crate) fn get_agent_texture(&self) -> &Texture2D {
        &self.agent_texture
    }
}
