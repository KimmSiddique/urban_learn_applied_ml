use crate::house_properties::{house_type::HouseType, position::Position};
use std::cmp::Ordering;

pub(crate) struct HouseDetails {
    position: Position,
    bedrooms: u8,
    bathrooms: u8,
    living_area_size: u32,
    land_size: u32,
    garage_spaces: u8,
    house_quality: u8, // Has to be between 1-10
    house_type: HouseType,
    age: u16,
    sale_price: f64,
}

impl PartialEq for HouseDetails {
    fn eq(&self, other: &Self) -> bool {
        self.sale_price == other.sale_price
    }
}

impl Eq for HouseDetails {}

impl PartialOrd for HouseDetails {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sale_price.partial_cmp(&other.sale_price)
    }
}

impl Ord for HouseDetails {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sale_price
            .partial_cmp(&other.sale_price)
            .expect("Could not compare house details")
    }
}

impl HouseDetails {
    pub(crate) fn get_bedrooms(&self) -> u8 {
        self.bedrooms
    }
    pub(crate) fn get_bathrooms(&self) -> u8 {
        self.bathrooms
    }
    pub(crate) fn get_living_area_size(&self) -> u32 {
        self.living_area_size
    }
    pub(crate) fn get_land_size(&self) -> u32 {
        self.land_size
    }
    pub(crate) fn get_house_age(&self) -> u16 {
        self.age
    }
    pub(crate) fn get_house_sale_price(&self) -> f64 {
        self.sale_price
    }
    pub(crate) fn get_house_type(&self) -> HouseType {
        self.house_type
    }
    pub(crate) fn get_house_quality(&self) -> u8 {
        self.house_quality
    }
    pub(crate) fn get_house_position(&self) -> Position {
        self.position
    }
    pub(crate) fn get_garage_spaces(&self) -> u8 {
        self.garage_spaces
    }

    pub(crate) fn new(
        position: Position,
        bedrooms: u8,
        bathrooms: u8,
        living_area_size: u32,
        land_size: u32,
        garage_spaces: u8,
        house_quality: u8, // Has to be between 1-10
        house_type: HouseType,
        age: u16,
        sale_price: f64,
    ) -> Self {
        Self {
            position,
            bedrooms,
            bathrooms,
            living_area_size,
            land_size,
            garage_spaces,
            house_quality,
            house_type,
            age,
            sale_price,
        }
    }

    pub(crate) fn generate_random_land_size(housetype: HouseType) -> u32 {
        let prob = rand::random_range(0..100);
        let landsize;

        match housetype {
            HouseType::Detached => {
                if (0..10).contains(&prob) {
                    landsize = rand::random_range(250..400);
                } else if (10..40).contains(&prob) {
                    landsize = rand::random_range(400..600);
                } else if (40..70).contains(&prob) {
                    landsize = rand::random_range(600..800);
                } else if (70..85).contains(&prob) {
                    landsize = rand::random_range(800..1000)
                } else if (85..95).contains(&prob) {
                    landsize = rand::random_range(1000..1500);
                } else {
                    landsize = rand::random_range(1500..2500);
                }
            }

            HouseType::SemiDetached => {
                if (0..15).contains(&prob) {
                    landsize = rand::random_range(250..400);
                } else if (15..55).contains(&prob) {
                    landsize = rand::random_range(400..600);
                } else if (55..85).contains(&prob) {
                    landsize = rand::random_range(600..800);
                } else if (85..97).contains(&prob) {
                    landsize = rand::random_range(800..1000)
                } else {
                    landsize = rand::random_range(1000..1500);
                }
            }

            HouseType::TownHouse => {
                if (0..95).contains(&prob) {
                    landsize = rand::random_range(400..600);
                } else {
                    landsize = rand::random_range(600..800);
                }
            }
        }

        landsize
    }

    pub(crate) fn generate_living_area_size(landsize: u32, housetype: HouseType) -> u32 {
        let prob = rand::random_range(0..100);

        match housetype {
            HouseType::Detached => {
                if (250..400).contains(&landsize) {
                    if prob < 15 {
                        rand::random_range(60..90)
                    } else if prob < 55 {
                        rand::random_range(90..120)
                    } else if prob < 90 {
                        rand::random_range(120..160)
                    } else {
                        rand::random_range(160..191)
                    }
                } else if (400..600).contains(&landsize) {
                    if prob < 10 {
                        rand::random_range(70..100)
                    } else if prob < 45 {
                        rand::random_range(100..140)
                    } else if prob < 85 {
                        rand::random_range(140..180)
                    } else {
                        rand::random_range(180..221)
                    }
                } else if (600..800).contains(&landsize) {
                    if prob < 8 {
                        rand::random_range(80..110)
                    } else if prob < 30 {
                        rand::random_range(110..140)
                    } else if prob < 70 {
                        rand::random_range(140..180)
                    } else if prob < 92 {
                        rand::random_range(180..220)
                    } else {
                        rand::random_range(220..281)
                    }
                } else if (800..1000).contains(&landsize) {
                    if prob < 7 {
                        rand::random_range(90..120)
                    } else if prob < 27 {
                        rand::random_range(120..160)
                    } else if prob < 65 {
                        rand::random_range(160..200)
                    } else if prob < 90 {
                        rand::random_range(200..250)
                    } else {
                        rand::random_range(250..321)
                    }
                } else if (1000..1500).contains(&landsize) {
                    if prob < 5 {
                        rand::random_range(90..130)
                    } else if prob < 25 {
                        rand::random_range(130..180)
                    } else if prob < 60 {
                        rand::random_range(180..230)
                    } else if prob < 88 {
                        rand::random_range(230..300)
                    } else {
                        rand::random_range(300..381)
                    }
                } else {
                    // 1500+ m²
                    if prob < 5 {
                        rand::random_range(100..150)
                    } else if prob < 20 {
                        rand::random_range(150..200)
                    } else if prob < 50 {
                        rand::random_range(200..270)
                    } else if prob < 80 {
                        rand::random_range(270..350)
                    } else {
                        rand::random_range(350..451)
                    }
                }
            }

            HouseType::SemiDetached => {
                if (250..400).contains(&landsize) {
                    if prob < 15 {
                        rand::random_range(60..90)
                    } else if prob < 60 {
                        rand::random_range(90..120)
                    } else if prob < 90 {
                        rand::random_range(120..150)
                    } else {
                        rand::random_range(150..181)
                    }
                } else if (400..600).contains(&landsize) {
                    if prob < 10 {
                        rand::random_range(70..100)
                    } else if prob < 45 {
                        rand::random_range(100..130)
                    } else if prob < 85 {
                        rand::random_range(130..170)
                    } else {
                        rand::random_range(170..211)
                    }
                } else if (600..800).contains(&landsize) {
                    if prob < 10 {
                        rand::random_range(80..110)
                    } else if prob < 45 {
                        rand::random_range(110..150)
                    } else if prob < 85 {
                        rand::random_range(150..190)
                    } else {
                        rand::random_range(190..231)
                    }
                } else if (800..1000).contains(&landsize) {
                    if prob < 10 {
                        rand::random_range(90..130)
                    } else if prob < 45 {
                        rand::random_range(130..170)
                    } else if prob < 85 {
                        rand::random_range(170..210)
                    } else {
                        rand::random_range(210..261)
                    }
                } else {
                    if prob < 10 {
                        rand::random_range(100..140)
                    } else if prob < 40 {
                        rand::random_range(140..180)
                    } else if prob < 80 {
                        rand::random_range(180..230)
                    } else {
                        rand::random_range(230..301)
                    }
                }
            }

            HouseType::TownHouse => {
                if (100..250).contains(&landsize) {
                    if prob < 15 {
                        rand::random_range(50..80)
                    } else if prob < 50 {
                        rand::random_range(80..110)
                    } else if prob < 85 {
                        rand::random_range(110..140)
                    } else {
                        rand::random_range(140..181)
                    }
                } else if (250..400).contains(&landsize) {
                    if prob < 10 {
                        rand::random_range(60..90)
                    } else if prob < 40 {
                        rand::random_range(90..120)
                    } else if prob < 80 {
                        rand::random_range(120..160)
                    } else {
                        rand::random_range(160..201)
                    }
                } else if (400..600).contains(&landsize) {
                    if prob < 10 {
                        rand::random_range(70..100)
                    } else if prob < 40 {
                        rand::random_range(100..140)
                    } else if prob < 80 {
                        rand::random_range(140..180)
                    } else {
                        rand::random_range(180..221)
                    }
                } else if (600..800).contains(&landsize) {
                    if prob < 10 {
                        rand::random_range(90..130)
                    } else if prob < 45 {
                        rand::random_range(130..170)
                    } else if prob < 85 {
                        rand::random_range(170..210)
                    } else {
                        rand::random_range(210..251)
                    }
                } else {
                    if prob < 10 {
                        rand::random_range(100..140)
                    } else if prob < 40 {
                        rand::random_range(140..180)
                    } else if prob < 80 {
                        rand::random_range(180..230)
                    } else {
                        rand::random_range(230..281)
                    }
                }
            }
        }
    }

    pub(crate) fn generate_bedrooms(living_area: u32) -> u8 {
        let prob = rand::random_range(0..100);

        if (50..80).contains(&living_area) {
            if prob < 60 {
                1
            } else if prob < 95 {
                2
            } else {
                3
            }
        } else if (80..120).contains(&living_area) {
            if prob < 10 {
                1
            } else if prob < 65 {
                2
            } else if prob < 95 {
                3
            } else {
                4
            }
        } else if (120..160).contains(&living_area) {
            if prob < 15 {
                2
            } else if prob < 70 {
                3
            } else if prob < 95 {
                4
            } else {
                5
            }
        } else if (160..200).contains(&living_area) {
            if prob < 5 {
                2
            } else if prob < 35 {
                3
            } else if prob < 85 {
                4
            } else {
                5
            }
        } else if (200..250).contains(&living_area) {
            if prob < 15 {
                3
            } else if prob < 60 {
                4
            } else if prob < 90 {
                5
            } else {
                6
            }
        } else if (250..300).contains(&living_area) {
            if prob < 5 {
                3
            } else if prob < 30 {
                4
            } else if prob < 70 {
                5
            } else if prob < 95 {
                6
            } else {
                7
            }
        } else if (300..400).contains(&living_area) {
            if prob < 15 {
                4
            } else if prob < 50 {
                5
            } else if prob < 80 {
                6
            } else if prob < 95 {
                7
            } else {
                8
            }
        } else {
            // 400+ m²
            if prob < 5 {
                4
            } else if prob < 25 {
                5
            } else if prob < 55 {
                6
            } else if prob < 80 {
                7
            } else if prob < 95 {
                8
            } else {
                9
            }
        }
    }

    pub(crate) fn generate_house_quality() -> u8 {
        let prob = rand::random_range(0..100);

        if prob < 1 {
            1
        } else if prob < 3 {
            2
        } else if prob < 9 {
            3
        } else if prob < 21 {
            4
        } else if prob < 43 {
            5
        } else if prob < 68 {
            6
        } else if prob < 86 {
            7
        } else if prob < 95 {
            8
        } else if prob < 99 {
            9
        } else {
            10
        }
    }

    pub(crate) fn generate_bathrooms(bedrooms: u8, house_quality: u8) -> u8 {
        let prob = rand::random_range(0..100);

        let mut bathrooms = match bedrooms {
            0..=1 => 1,

            2 => {
                if prob < 75 {
                    1
                } else {
                    2
                }
            }

            3 => {
                if prob < 50 {
                    1
                } else if prob < 95 {
                    2
                } else {
                    3
                }
            }

            4 => {
                if prob < 15 {
                    1
                } else if prob < 70 {
                    2
                } else if prob < 95 {
                    3
                } else {
                    4
                }
            }

            5 => {
                if prob < 10 {
                    2
                } else if prob < 60 {
                    3
                } else if prob < 90 {
                    4
                } else {
                    5
                }
            }

            _ => {
                if prob < 15 {
                    2
                } else if prob < 55 {
                    3
                } else if prob < 85 {
                    4
                } else {
                    5
                }
            }
        };

        // High-quality houses have a greater chance of an extra bathroom.
        if house_quality >= 8 && rand::random_range(0..100) < 30 {
            bathrooms += 1;
        }

        // Poor-quality houses occasionally have fewer bathrooms.
        if house_quality <= 3 && bathrooms > 1 && rand::random_range(0..100) < 25 {
            bathrooms -= 1;
        }

        bathrooms
    }

    pub(crate) fn generate_garage_spaces(
        land_size: u32,
        living_area_size: u32,
        house_type: HouseType,
    ) -> u8 {
        let prob = rand::random_range(0..100);

        let mut spaces = match land_size {
            0..250 => {
                if prob < 60 {
                    0
                } else {
                    1
                }
            }

            250..400 => {
                if prob < 25 {
                    0
                } else if prob < 85 {
                    1
                } else {
                    2
                }
            }

            400..600 => {
                if prob < 10 {
                    0
                } else if prob < 55 {
                    1
                } else if prob < 95 {
                    2
                } else {
                    3
                }
            }

            600..800 => {
                if prob < 5 {
                    0
                } else if prob < 30 {
                    1
                } else if prob < 85 {
                    2
                } else {
                    3
                }
            }

            800..1000 => {
                if prob < 15 {
                    1
                } else if prob < 70 {
                    2
                } else if prob < 95 {
                    3
                } else {
                    4
                }
            }

            1000..1500 => {
                if prob < 10 {
                    1
                } else if prob < 55 {
                    2
                } else if prob < 90 {
                    3
                } else {
                    4
                }
            }

            _ => {
                if prob < 10 {
                    1
                } else if prob < 45 {
                    2
                } else if prob < 80 {
                    3
                } else {
                    4
                }
            }
        };

        // Large houses are somewhat more likely to need more garage capacity.
        if living_area_size >= 300 && spaces < 4 && rand::random_range(0..100) < 25 {
            spaces += 1;
        }

        // Townhouses are less likely to have large garages.
        if house_type == HouseType::TownHouse && spaces > 1 && rand::random_range(0..100) < 50 {
            spaces -= 1;
        }

        spaces
    }

    pub(crate) fn generate_sale_price(
        bedrooms: u8,
        bathrooms: u8,
        living_area_size: u32,
        land_size: u32,
        garage_spaces: u8,
        house_quality: u8,
        house_type: HouseType,
        age: u16,
    ) -> f64 {
        assert!((1..=10).contains(&house_quality));

        let land_value = land_size as f64 * 100.0;
        let base_building_value = living_area_size as f64 * 850.0;

        let quality_factor = match house_quality {
            1 => 0.55,
            2 => 0.65,
            3 => 0.75,
            4 => 0.87,
            5 => 1.00,
            6 => 1.10,
            7 => 1.22,
            8 => 1.38,
            9 => 1.58,
            10 => 1.80,
            _ => unreachable!(),
        };

        let age_factor = match age {
            0..=5 => 1.08,
            6..=15 => 1.04,
            16..=30 => 1.00,
            31..=50 => 0.93,
            51..=75 => 0.86,
            76..=100 => 0.82,
            101..=125 if house_quality >= 8 => 0.85,
            126.. if house_quality >= 8 => 0.90,
            101..=125 => 0.77,
            _ => 0.72,
        };

        let type_factor = match house_type {
            HouseType::Detached => 1.00,
            HouseType::SemiDetached => 0.95,
            HouseType::TownHouse => 0.91,
        };

        let building_value = base_building_value * quality_factor * age_factor * type_factor;

        let bedroom_adjustment = match bedrooms {
            0 | 1 => -12000.0,
            2 => -5000.0,
            3 => 0.0,
            4 => 8000.0,
            5 => 14000.0,
            6 => 18000.0,
            _ => 20000.0,
        };

        let bathroom_adjustment = match bathrooms {
            0 | 1 => 0.0,
            2 => 12000.0,
            3 => 20000.0,
            4 => 26000.0,
            5 => 30000.0,
            6 => 32000.0,
            _ => 33000.0,
        };

        let garage_adjustment = match garage_spaces {
            0 => -8000.0,
            1 => 0.0,
            2 => 10000.0,
            3 => 16000.0,
            4 => 20000.0,
            5 => 22000.0,
            6 => 23000.0,
            _ => 24000.0,
        };

        let density_penalty = if bedrooms > 0 && (living_area_size as f64 / bedrooms as f64) < 25.0
        {
            -10000.0
        } else {
            0.0
        };

        let fundamental_value = (land_value
            + building_value
            + bedroom_adjustment
            + bathroom_adjustment
            + garage_adjustment
            + density_penalty)
            .max(10000.0);

        // Hidden desirability represents unobserved house characteristics.
        // Averaging three random values makes scores near 0.5 more common.
        // This score will not be included in the ML dataset.
        let desirability_score: f64 = (rand::random_range(0.0..1.0)
            + rand::random_range(0.0..1.0)
            + rand::random_range(0.0..1.0))
            / 3.0;

        // Converts the hidden score into a multiplier between 0.92 and 1.08.
        // Score 0.5 produces no price adjustment.
        let desirability_factor = 1.0 + 0.16 * (desirability_score - 0.5);

        // Random market noise represents unpredictable sale conditions.
        // Averaging three uniform random values gives a distribution
        // concentrated around 0, with possible deviations of roughly ±3%.
        // This is not a true normal distribution.
        let market_noise: f64 = (rand::random_range(-0.03..0.03)
            + rand::random_range(-0.03..0.03)
            + rand::random_range(-0.03..0.03))
            / 3.0;

        let market_factor = 1.0 + market_noise;

        (fundamental_value * desirability_factor * market_factor).round()
    }

    pub(crate) fn generate_age() -> u16 {
        let prob = rand::random_range(0..100);

        if prob < 8 {
            rand::random_range(0..=5)
        } else if prob < 23 {
            rand::random_range(6..=15)
        } else if prob < 48 {
            rand::random_range(16..=30)
        } else if prob < 73 {
            rand::random_range(31..=50)
        } else if prob < 88 {
            rand::random_range(51..=75)
        } else if prob < 96 {
            rand::random_range(76..=100)
        } else {
            rand::random_range(101..=150)
        }
    }

    pub(crate) fn generate_random_house_details(
        position: Position,
        house_evaulation_price: f64,
    ) -> Self {
        let housetype = HouseType::get_random_housetype();
        let landsize = Self::generate_random_land_size(housetype);
        let living_area = Self::generate_living_area_size(landsize, housetype);
        let bedrooms = Self::generate_bedrooms(living_area);
        let quality = Self::generate_house_quality();
        let bathrooms = Self::generate_bathrooms(bedrooms, quality);
        let garage_spaces = Self::generate_garage_spaces(landsize, living_area, housetype);
        let age = Self::generate_age();
        let sale_price = Self::generate_sale_price(
            bedrooms,
            bathrooms,
            living_area,
            landsize,
            garage_spaces,
            quality,
            housetype,
            age,
        );

        Self {
            position,
            bedrooms: bedrooms,
            bathrooms: bathrooms,
            living_area_size: living_area,
            land_size: landsize,
            garage_spaces: garage_spaces,
            house_quality: quality,
            house_type: housetype,
            age: age,
            sale_price: sale_price + house_evaulation_price,
        }
    }
}
