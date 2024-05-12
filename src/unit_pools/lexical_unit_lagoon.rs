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
    pub artifact_art: LexicalUnitPool,
    pub artifact_dedication: LexicalUnitPool,

    pub scene_actors: LexicalUnitPool,
    pub scene_verbs: LexicalUnitPool,
    pub scene_objects: LexicalUnitPool,

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
    pub currency_icon_insignia: LexicalUnitPool,
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
        let mut artifact_art: LexicalUnitPool = Default::default();
        let mut artifact_dedication: LexicalUnitPool = Default::default();

        let mut scene_actors: LexicalUnitPool = Default::default();
        let mut scene_verbs: LexicalUnitPool = Default::default();
        let mut scene_objects: LexicalUnitPool = Default::default();

        let mut timeline_qualifiers: LexicalUnitPool = Default::default();

        let mut ship_prefixes: LexicalUnitPool = Default::default();
        let mut ship_long_names: LexicalUnitPool = Default::default();
        let mut ship_sizes: LexicalUnitPool = Default::default();
        let mut ship_type_ss: LexicalUnitPool = Default::default();
        let mut ship_type_xs: LexicalUnitPool = Default::default();
        let mut ship_type_s: LexicalUnitPool = Default::default();
        let mut ship_type_m: LexicalUnitPool = Default::default();
        let mut ship_type_l: LexicalUnitPool = Default::default();
        let mut ship_type_xl: LexicalUnitPool = Default::default();
        let mut ship_type_xxl: LexicalUnitPool = Default::default();
        let mut ship_type_u: LexicalUnitPool = Default::default();
        let mut ship_type_xu: LexicalUnitPool = Default::default();
        let mut ship_type_t: LexicalUnitPool = Default::default();
        let mut ship_fame: LexicalUnitPool = Default::default();

        let mut currency_real_names: LexicalUnitPool = Default::default();
        let mut currency_real_fractional_names: LexicalUnitPool = Default::default();
        let mut currency_endings: LexicalUnitPool = Default::default();
        let mut currency_second_word: LexicalUnitPool = Default::default();
        let mut currency_metals: LexicalUnitPool = Default::default();
        let mut currency_icon_inscription: LexicalUnitPool = Default::default();
        let mut currency_icon_figure: LexicalUnitPool = Default::default();
        let mut currency_icon_partial: LexicalUnitPool = Default::default();
        let mut currency_icon_incuse: LexicalUnitPool = Default::default();
        let mut currency_icon_insignia: LexicalUnitPool = Default::default();
        let mut currency_icon_mythical_creature: LexicalUnitPool = Default::default();
        let mut currency_icon_animal: LexicalUnitPool = Default::default();
        let mut currency_icon_plant: LexicalUnitPool = Default::default();

        let mut metals_list: LexicalUnitPool = Default::default();
        let mut metals_alloy_list: LexicalUnitPool = Default::default();

        let mut government_name0: LexicalUnitPool = Default::default();
        let mut government_name1: LexicalUnitPool = Default::default();
        let mut government_name_monarchy: LexicalUnitPool = Default::default();
        let mut rep_or_state_list: LexicalUnitPool = Default::default();

        let mut story_empire_interactions: LexicalUnitPool = Default::default();
        let mut story_government_type_adj_0: LexicalUnitPool = Default::default();
        let mut story_government_type_adj_01: LexicalUnitPool = Default::default();
        let mut story_government_type_adj_1: LexicalUnitPool = Default::default();
        let mut story_government_type_sec_0: LexicalUnitPool = Default::default();
        let mut story_government_type_sec_01: LexicalUnitPool = Default::default();
        let mut story_government_type_sec_1: LexicalUnitPool = Default::default();
        let mut story_government_leader_title_0_reli: LexicalUnitPool = Default::default();
        let mut story_government_leader_title_0_cor: LexicalUnitPool = Default::default();
        let mut story_government_leader_title_0_mon: LexicalUnitPool = Default::default();
        let mut story_government_leader_title_01: LexicalUnitPool = Default::default();
        let mut story_government_leader_title_1: LexicalUnitPool = Default::default();

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
                            place_relatives = LexicalUnitPool::from(entry);
                        },
                        "place_noun" => {
                            place_noun = LexicalUnitPool::from(entry);
                        },
                        "place_top_eng_nouns" => {
                            place_top_eng_nouns = LexicalUnitPool::from(entry);
                        },
                        "place_object" => {
                            place_object = LexicalUnitPool::from(entry);
                        },
                        "place_single" => {
                            place_single = LexicalUnitPool::from(entry);
                        },
                        "general_comp0" => {
                            general_comp0 = LexicalUnitPool::from(entry);
                        },
                        "general_comp1" => {
                            general_comp1 = LexicalUnitPool::from(entry);
                        },
                        "people_first_name" => {
                            people_first_name = LexicalUnitPool::from(entry);
                        },
                        "people_last_name" => {
                            people_last_name = LexicalUnitPool::from(entry);
                        },
                        "people_nickname" => {
                            people_nickname = LexicalUnitPool::from(entry);
                        },
                        "people_title" => {
                            people_title = LexicalUnitPool::from(entry);
                        },
                        "people_skill" => {
                            people_skill = LexicalUnitPool::from(entry);
                        },
                        "people_skill_level" => {
                            people_skill_level = LexicalUnitPool::from(entry);
                        },
                        "people_trait_ck2" => {
                            people_trait_ck2 = LexicalUnitPool::from(entry);
                        },
                        "people_trait_ck3" => {
                            people_trait_ck3 = LexicalUnitPool::from(entry);
                        },
                        "artifact_type" => {
                            artifact_type = LexicalUnitPool::from(entry);
                        },
                        "artifact_adjective" => {
                            artifact_adjective = LexicalUnitPool::from(entry);
                        },
                        "artifact_material" => {
                            artifact_material = LexicalUnitPool::from(entry);
                        },
                        "artifact_quality" => {
                            artifact_quality = LexicalUnitPool::from(entry);
                        },
                        "artifact_art" => {
                            artifact_art = LexicalUnitPool::from(entry);
                        },
                        "artifact_dedication" => {
                            artifact_dedication = LexicalUnitPool::from(entry);
                        },
                        "scene_actors" => {
                            scene_actors = LexicalUnitPool::from(entry);
                        },
                        "scene_verbs" => {
                            scene_verbs = LexicalUnitPool::from(entry);
                        },
                        "scene_objects" => {
                            scene_objects= LexicalUnitPool::from(entry);
                        },
                        "timeline_qualifiers" => {
                            timeline_qualifiers = LexicalUnitPool::from(entry);
                        },
                        "ship_prefixes" => {
                            ship_prefixes = LexicalUnitPool::from(entry);
                        },
                        "ship_long_names" => {
                            ship_long_names = LexicalUnitPool::from(entry);
                        },
                        "ship_sizes" => {
                            ship_sizes = LexicalUnitPool::from(entry);
                        },
                        "ship_type_ss" => {
                            ship_type_ss = LexicalUnitPool::from(entry);
                        },
                        "ship_type_xs" => {
                            ship_type_xs = LexicalUnitPool::from(entry);
                        },
                        "ship_type_s" => {
                            ship_type_s = LexicalUnitPool::from(entry);
                        },
                        "ship_type_m" => {
                            ship_type_m = LexicalUnitPool::from(entry);
                        },
                        "ship_type_l" => {
                            ship_type_l = LexicalUnitPool::from(entry);
                        },
                        "ship_type_xl" => {
                            ship_type_xl = LexicalUnitPool::from(entry);
                        },
                        "ship_type_xxl" => {
                            ship_type_xxl = LexicalUnitPool::from(entry);
                        },
                        "ship_type_u" => {
                            ship_type_u = LexicalUnitPool::from(entry);
                        },
                        "ship_type_xu" => {
                            ship_type_xu = LexicalUnitPool::from(entry);
                        },
                        "ship_type_t" => {
                            ship_type_t = LexicalUnitPool::from(entry);
                        },
                        "ship_fame" => {
                            ship_fame = LexicalUnitPool::from(entry);
                        },
                        "currency_real_names" => {
                            currency_real_names = LexicalUnitPool::from(entry);
                        },
                        "currency_real_fractional_names" => {
                            currency_real_fractional_names = LexicalUnitPool::from(entry);
                        },
                        "currency_endings" => {
                            currency_endings = LexicalUnitPool::from(entry);
                        },
                        "currency_second_word" => {
                            currency_second_word = LexicalUnitPool::from(entry);
                        },
                        "currency_metals" => {
                            currency_metals = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_inscription" => {
                            currency_icon_inscription = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_figure" => {
                            currency_icon_figure = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_partial" => {
                            currency_icon_partial = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_incuse" => {
                            currency_icon_incuse = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_insignia" => {
                            currency_icon_insignia = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_mythical_creature" => {
                            currency_icon_mythical_creature = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_animal" => {
                            currency_icon_animal = LexicalUnitPool::from(entry);
                        },
                        "currency_icon_plant" => {
                            currency_icon_plant = LexicalUnitPool::from(entry);
                        },
                        "metals_list" => {
                            metals_list = LexicalUnitPool::from(entry);
                        },
                        "metals_alloy_list" => {
                            metals_alloy_list = LexicalUnitPool::from(entry);
                        },
                        "government_name0" => {
                            government_name0 = LexicalUnitPool::from(entry);
                        },
                        "government_name1" => {
                            government_name1 = LexicalUnitPool::from(entry);
                        },
                        "government_name_monarchy" => {
                            government_name_monarchy = LexicalUnitPool::from(entry);
                        },
                        "rep_or_state_list" => {
                            rep_or_state_list = LexicalUnitPool::from(entry);
                        },
                        "story_empire_interactions" => {
                            story_empire_interactions = LexicalUnitPool::from(entry);
                        },
                        "story_government_type_adj_0" => {
                            story_government_type_adj_0 = LexicalUnitPool::from(entry);
                        },
                        "story_government_type_adj_01" => {
                            story_government_type_adj_01 = LexicalUnitPool::from(entry);
                        },
                        "story_government_type_adj_1" => {
                            story_government_type_adj_1 = LexicalUnitPool::from(entry);
                        },
                        "story_government_type_sec_0" => {
                            story_government_type_sec_0 = LexicalUnitPool::from(entry);
                        },
                        "story_government_type_sec_01" => {
                            story_government_type_sec_01 = LexicalUnitPool::from(entry);
                        },
                        "story_government_type_sec_1" => {
                            story_government_type_sec_1 = LexicalUnitPool::from(entry);
                        },
                        "story_government_leader_title_0_reli" => {
                            story_government_leader_title_0_reli = LexicalUnitPool::from(entry);
                        },
                        "story_government_leader_title_0_cor" => {
                            story_government_leader_title_0_cor = LexicalUnitPool::from(entry);
                        },
                        "story_government_leader_title_0_mon" => {
                            story_government_leader_title_0_mon = LexicalUnitPool::from(entry);
                        },
                        "story_government_leader_title_01" => {
                            story_government_leader_title_01 = LexicalUnitPool::from(entry);
                        },
                        "story_government_leader_title_1" => {
                            story_government_leader_title_1 = LexicalUnitPool::from(entry);
                        },
                        _ => {
                            Err(Error::other(format!("Undeclared json list. {} (in {:?}) is not implemented!", entry.0, gen_lib_path )))
                        }?
                    };
                }
                Ok(LexicalUnitLagoon { abc, place_relatives, place_noun, place_top_eng_nouns, place_object, place_single, general_comp0, general_comp1, people_first_name, people_last_name, people_nickname, people_title, people_skill, people_skill_level, people_trait_ck2, people_trait_ck3, artifact_type, artifact_adjective, artifact_material, artifact_quality, artifact_art, artifact_dedication, scene_actors, scene_verbs, scene_objects, timeline_qualifiers, ship_prefixes, ship_long_names, ship_sizes, ship_type_ss, ship_type_xs, ship_type_s, ship_type_m, ship_type_l, ship_type_xl, ship_type_xxl, ship_type_u, ship_type_xu, ship_type_t, ship_fame, currency_real_names, currency_real_fractional_names, currency_endings, currency_second_word, currency_metals, currency_icon_inscription, currency_icon_figure, currency_icon_partial, currency_icon_incuse, currency_icon_insignia, currency_icon_mythical_creature, currency_icon_animal, currency_icon_plant, metals_list, metals_alloys_list: metals_alloy_list, government_name0, government_name1, government_name_monarchy, rep_or_state_list, story_empire_interactions, story_government_type_adj_0, story_government_type_adj_01, story_government_type_adj_1, story_government_type_sec_0, story_government_type_sec_01, story_government_type_sec_1, story_government_leader_title_0_reli, story_government_leader_title_0_cor, story_government_leader_title_0_mon, story_government_leader_title_01, story_government_leader_title_1 })
            },
            Err(error) => {
                Err(Error::new(ErrorKind::Other, error))
            },
        }
    }
}
