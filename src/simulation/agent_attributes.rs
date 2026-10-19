use crate::house_properties::house::House;
use smartcore::ensemble::random_forest_regressor::{
    RandomForestRegressor, RandomForestRegressorParameters,
};
use smartcore::linalg::basic::matrix::DenseMatrix;
use std::error::Error;

type HouseModel = RandomForestRegressor<f64, f64, DenseMatrix<f64>, Vec<f64>>;

#[derive(Debug, Clone, Copy)]
pub(crate) enum AgentMode {
    RandomExplorationMode,
    SystematicExplorationMode,
    SmartExplorationMode,
}
pub(crate) struct AgentAttributes {
    agent_mode: AgentMode,
    houses: Vec<House>,
    model: Option<HouseModel>,
    agent_energy: u32,
}

impl AgentAttributes {
    pub(crate) fn create_dense_matrix_and_target_values(
        &self,
    ) -> Result<(DenseMatrix<f64>, Vec<f64>), Box<dyn Error>> {
        let mut target_values = Vec::new();
        let mut features_vec: Vec<Vec<f64>> = Vec::new();
        for house in self.houses.iter() {
            let (feature, price) = house.featurify();
            features_vec.push(feature);
            target_values.push(price);
        }

        let feature_matrix = DenseMatrix::from_2d_vec(&features_vec)?;
        Ok((feature_matrix, target_values))
    }

    pub(crate) fn set_regression_model(
        &mut self,
        feature_matrix: DenseMatrix<f64>,
        target_values: Vec<f64>,
    ) {
        self.model = Some(
            HouseModel::fit(
                &feature_matrix,
                &target_values,
                RandomForestRegressorParameters::default(),
            )
            .expect("Could not train model"),
        );
    }

    pub(crate) fn new(agent_mode: AgentMode, agent_energy: u32) -> Self {
        Self {
            agent_mode,
            houses: Vec::new(),
            model: None,
            agent_energy,
        }
    }

    pub(crate) fn predict(&self, features: &DenseMatrix<f64>) -> Option<Vec<f64>> {
        let model = self.model.as_ref()?;
        Some(model.predict(&features).ok()?)
    }

    pub(crate) fn get_agent_mode(&self) -> AgentMode {
        self.agent_mode
    }

    pub(crate) fn get_agent_energy(&self) -> u32 {
        self.agent_energy
    }

    pub(crate) fn decrement_agent_energy(&mut self) {
        self.agent_energy -= 1;
    }
}
