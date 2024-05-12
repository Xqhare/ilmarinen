use std::{io::{Error, ErrorKind}, path::Path};

use crate::jisard::read_json;

use super::lexical_unit_pool::LexicalUnitPool;

#[derive(Debug, Clone, Default)]
pub struct LexicalUnitLagoon {
    pub abc: LexicalUnitPool,

    pub place_relatives: LexicalUnitPool,
    pub place_noun: LexicalUnitPool,
    /// The most used english nouns
    pub place_top_eng_nouns: LexicalUnitPool,
    pub place_object: LexicalUnitPool,
    pub place_single: LexicalUnitPool,

    pub general_comp0: LexicalUnitPool,
    pub general_comp1: LexicalUnitPool,

    pub people_first_name: LexicalUnitPool,
    pub people_last_name: LexicalUnitPool,
    pub people_nickname: LexicalUnitPool,
    pub people_title: LexicalUnitPool,
    pub people_skill: LexicalUnitPool,
    pub people_skill_level: LexicalUnitPool,
    pub people_trait_ck2: LexicalUnitPool,
    pub people_trait_ck3: LexicalUnitPool,

    pub artifact_type: LexicalUnitPool,
    pub artifact_adjective: LexicalUnitPool,
    pub artifact_material: LexicalUnitPool,
    pub artifact_quality: LexicalUnitPool,
    pub artifact_dedication: LexicalUnitPool,

    pub scene_actors: LexicalUnitPool,

    pub timeline_qualifiers: LexicalUnitPool,

    pub ship_prefixes: LexicalUnitPool,
    pub ship_long_names: LexicalUnitPool,
    pub ship_sizes: LexicalUnitPool,
    pub ship_type_ss: LexicalUnitPool,
    pub ship_type_xs: LexicalUnitPool,
    pub ship_type_s: LexicalUnitPool,
    pub ship_type_m: LexicalUnitPool,
    pub ship_type_l: LexicalUnitPool,
    pub ship_type_xl: LexicalUnitPool,
    pub ship_type_xxl: LexicalUnitPool,
    pub ship_type_u: LexicalUnitPool,
    pub ship_type_xu: LexicalUnitPool,
    pub ship_type_t: LexicalUnitPool,
    pub ship_fame: LexicalUnitPool,

    pub currency_real_names: LexicalUnitPool,
    pub currency_real_fractional_names: LexicalUnitPool,
    pub currency_endings: LexicalUnitPool,
    pub currency_second_word: LexicalUnitPool,
    pub currency_metals: LexicalUnitPool,
    pub currency_icon_inscription: LexicalUnitPool,
    pub currency_icon_figure: LexicalUnitPool,
    pub currency_icon_partial: LexicalUnitPool,
    pub currency_icon_incuse: LexicalUnitPool,
    pub currency_icon_mythical_creature: LexicalUnitPool,
    pub currency_icon_animal: LexicalUnitPool,
    pub currency_icon_plant: LexicalUnitPool,

    pub metals_list: LexicalUnitPool,
    pub metals_alloys_list: LexicalUnitPool,

    pub government_name0: LexicalUnitPool,
    pub government_name1: LexicalUnitPool,
    pub government_name_monarchy: LexicalUnitPool,
    pub rep_or_state_list: LexicalUnitPool,

    pub story_empire_interactions: LexicalUnitPool,
    pub story_government_type_adj_0: LexicalUnitPool,
    pub story_government_type_adj_01: LexicalUnitPool,
    pub story_government_type_adj_1: LexicalUnitPool,
    pub story_government_type_sec_0: LexicalUnitPool,
    pub story_government_type_sec_01: LexicalUnitPool,
    pub story_government_type_sec_1: LexicalUnitPool,
    pub story_government_leader_title_0_reli: LexicalUnitPool,
    pub story_government_leader_title_0_cor: LexicalUnitPool,
    pub story_government_leader_title_0_mon: LexicalUnitPool,
    pub story_government_leader_title_01: LexicalUnitPool,
    pub story_government_leader_title_1: LexicalUnitPool,
}

impl LexicalUnitLagoon {
    pub fn new(gen_lib_path: &Path) -> Result<LexicalUnitLagoon, Error> {
        let mut abc: LexicalUnitPool = Default::default();

        let mut place_relatives: LexicalUnitPool = Default::default(); 
        let mut place_noun: LexicalUnitPool = Default::default();
        let mut place_top_eng_nouns: LexicalUnitPool = Default::default();
        let mut place_object: LexicalUnitPool = Default::default();
        let mut place_single: LexicalUnitPool = Default::default();

        let mut general_comp0: LexicalUnitPool = Default::default();
        let mut general_comp1: LexicalUnitPool = Default::default();

        let mut people_first_name: LexicalUnitPool = Default::default();
        let mut people_last_name: LexicalUnitPool = Default::default();
        let mut people_nickname: LexicalUnitPool = Default::default();
        let mut people_title: LexicalUnitPool = Default::default();
        let mut people_skill: LexicalUnitPool = Default::default();
        let mut people_skill_level: LexicalUnitPool = Default::default();
        let mut people_trait_ck2: LexicalUnitPool = Default::default();
        let mut people_trait_ck3: LexicalUnitPool = Default::default();

        let mut artifact_type: LexicalUnitPool = Default::default();
        let mut artifact_adjective: LexicalUnitPool = Default::default();
        let mut artifact_material: LexicalUnitPool = Default::default();
        let mut artifact_quality: LexicalUnitPool = Default::default();
        let mut artifact_dedication: LexicalUnitPool = Default::default();

        let mut scene_actors: LexicalUnitPool = Default::default();

        let mut timeline_qualifiers: LexicalUnitPool = Default::default();
        match read_json(gen_lib_path) {
            Ok(data) => {
                for entry in data.entries() {
                    println!("{:?}", entry);
                    println!("");
                    match entry.0 {
                        "abc" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "place_relatives" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "place_noun" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "place_top_eng_nouns" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "place_object" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "place_single" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "general_comp0" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "general_comp1" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_first_name" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_last_name" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_nickname" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_title" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_skill" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_skill_level" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_trait_ck2" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "people_trait_ck3" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "artifact_type" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "artifact_adjective" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "artifact_material" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "artifact_quality" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "artifact_dedication" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "scene_actors" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        "timeline_qualifiers" => {
                            abc = LexicalUnitPool::from(entry);
                        },
                        _ => {
                            Err(Error::other(format!("Undeclared json list. {} (in {:?}) is not implemented!", entry.0, gen_lib_path )))
                        }?
                    };
                }
                Ok(LexicalUnitLagoon::default())
            },
            Err(error) => {
                Err(Error::new(ErrorKind::Other, error))
            },
        }
    }
}
