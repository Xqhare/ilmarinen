use std::{io::Error, path::Path};

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
    /// Contains all skill levels, low to ultra
    pub people_skill_level_all: LexicalUnitPool,
    pub people_skill_level_low: LexicalUnitPool,
    pub people_skill_level_medium: LexicalUnitPool,
    pub people_skill_level_high: LexicalUnitPool,
    pub people_skill_level_ultra: LexicalUnitPool,
    pub people_trait_ck2: LexicalUnitPool,
    pub people_trait_ck3: LexicalUnitPool,
    pub people_trait_rolls_ck2_mod: LexicalUnitPool,

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

    pub person_social_gender: LexicalUnitPool,
}

impl LexicalUnitLagoon {
    pub fn new(gen_lib_path: &Path) -> Result<LexicalUnitLagoon, Error> {
        let abc: LexicalUnitPool;

        let place_relatives: LexicalUnitPool;
        let place_noun: LexicalUnitPool;
        let place_top_eng_nouns: LexicalUnitPool;
        let place_object: LexicalUnitPool;
        let place_single: LexicalUnitPool;

        let general_comp0: LexicalUnitPool;
        let general_comp1: LexicalUnitPool;

        let people_first_name: LexicalUnitPool;
        let people_last_name: LexicalUnitPool;
        let people_nickname: LexicalUnitPool;
        let people_title: LexicalUnitPool;
        let people_skill: LexicalUnitPool;
        let people_skill_level_low: LexicalUnitPool;
        let people_skill_level_medium: LexicalUnitPool;
        let people_skill_level_high: LexicalUnitPool;
        let people_skill_level_ultra: LexicalUnitPool;
        let people_trait_ck2: LexicalUnitPool;
        let people_trait_ck3: LexicalUnitPool;
        let people_trait_rolls_ck2_mod: LexicalUnitPool;

        let artifact_type: LexicalUnitPool;
        let artifact_adjective: LexicalUnitPool;
        let artifact_material: LexicalUnitPool;
        let artifact_quality: LexicalUnitPool;
        let artifact_art: LexicalUnitPool;
        let artifact_dedication: LexicalUnitPool;

        let scene_actors: LexicalUnitPool;
        let scene_verbs: LexicalUnitPool;
        let scene_objects: LexicalUnitPool;

        let timeline_qualifiers: LexicalUnitPool;

        let ship_prefixes: LexicalUnitPool;
        let ship_long_names: LexicalUnitPool;
        let ship_sizes: LexicalUnitPool;
        let ship_type_ss: LexicalUnitPool;
        let ship_type_xs: LexicalUnitPool;
        let ship_type_s: LexicalUnitPool;
        let ship_type_m: LexicalUnitPool;
        let ship_type_l: LexicalUnitPool;
        let ship_type_xl: LexicalUnitPool;
        let ship_type_xxl: LexicalUnitPool;
        let ship_type_u: LexicalUnitPool;
        let ship_type_xu: LexicalUnitPool;
        let ship_type_t: LexicalUnitPool;
        let ship_fame: LexicalUnitPool;

        let currency_real_names: LexicalUnitPool;
        let currency_real_fractional_names: LexicalUnitPool;
        let currency_endings: LexicalUnitPool;
        let currency_second_word: LexicalUnitPool;
        let currency_metals: LexicalUnitPool;
        let currency_icon_inscription: LexicalUnitPool;
        let currency_icon_figure: LexicalUnitPool;
        let currency_icon_partial: LexicalUnitPool;
        let currency_icon_incuse: LexicalUnitPool;
        let currency_icon_insignia: LexicalUnitPool;
        let currency_icon_mythical_creature: LexicalUnitPool;
        let currency_icon_animal: LexicalUnitPool;
        let currency_icon_plant: LexicalUnitPool;

        let metals_list: LexicalUnitPool;
        let metals_alloy_list: LexicalUnitPool;

        let government_name0: LexicalUnitPool;
        let government_name1: LexicalUnitPool;
        let government_name_monarchy: LexicalUnitPool;
        let rep_or_state_list: LexicalUnitPool;

        let story_empire_interactions: LexicalUnitPool;
        let story_government_type_adj_0: LexicalUnitPool;
        let story_government_type_adj_01: LexicalUnitPool;
        let story_government_type_adj_1: LexicalUnitPool;
        let story_government_type_sec_0: LexicalUnitPool;
        let story_government_type_sec_01: LexicalUnitPool;
        let story_government_type_sec_1: LexicalUnitPool;
        let story_government_leader_title_0_reli: LexicalUnitPool;
        let story_government_leader_title_0_cor: LexicalUnitPool;
        let story_government_leader_title_0_mon: LexicalUnitPool;
        let story_government_leader_title_01: LexicalUnitPool;
        let story_government_leader_title_1: LexicalUnitPool;

        let person_social_gender: LexicalUnitPool;

        match read_json(gen_lib_path) {
            Ok(data) => {
                let obj = data.as_object().expect("JSON parent must be an object");
                abc = LexicalUnitPool::from((
                    "abc",
                    obj.get("abc").expect("Object must include key 'abc'"),
                ));
                place_relatives = LexicalUnitPool::from((
                    "place_relatives",
                    obj.get("place_relatives")
                        .expect("Object must include key 'place_relatives'"),
                ));
                place_noun = LexicalUnitPool::from((
                    "place_noun",
                    obj.get("place_noun")
                        .expect("Object must include key 'place_noun'"),
                ));
                place_top_eng_nouns = LexicalUnitPool::from((
                    "place_top_eng_nouns",
                    obj.get("place_top_eng_nouns")
                        .expect("Object must include key 'place_top_eng_nouns'"),
                ));
                place_object = LexicalUnitPool::from((
                    "place_object",
                    obj.get("place_object")
                        .expect("Object must include key 'place_object'"),
                ));
                place_single = LexicalUnitPool::from((
                    "place_single",
                    obj.get("place_single")
                        .expect("Object must include key 'place_single'"),
                ));
                general_comp0 = LexicalUnitPool::from((
                    "general_comp0",
                    obj.get("general_comp0")
                        .expect("Object must include key 'general_comp0'"),
                ));
                general_comp1 = LexicalUnitPool::from((
                    "general_comp1",
                    obj.get("general_comp1")
                        .expect("Object must include key 'general_comp1'"),
                ));
                people_first_name = LexicalUnitPool::from((
                    "people_first_name",
                    obj.get("people_first_name")
                        .expect("Object must include key 'people_first_name'"),
                ));
                people_last_name = LexicalUnitPool::from((
                    "people_last_name",
                    obj.get("people_last_name")
                        .expect("Object must include key 'people_last_name'"),
                ));
                people_nickname = LexicalUnitPool::from((
                    "people_nickname",
                    obj.get("people_nickname")
                        .expect("Object must include key 'people_nickname'"),
                ));
                people_title = LexicalUnitPool::from((
                    "people_title",
                    obj.get("people_title")
                        .expect("Object must include key 'people_title'"),
                ));
                people_skill = LexicalUnitPool::from((
                    "people_skill",
                    obj.get("people_skill")
                        .expect("Object must include key 'people_skill'"),
                ));
                people_skill_level_low = LexicalUnitPool::from((
                    "people_skill_level_low",
                    obj.get("people_skill_level_low")
                        .expect("Object must include key 'people_skill_level_low'"),
                ));
                people_skill_level_medium = LexicalUnitPool::from((
                    "people_skill_level_medium",
                    obj.get("people_skill_level_medium")
                        .expect("Object must include key 'people_skill_level_medium'"),
                ));
                people_skill_level_high = LexicalUnitPool::from((
                    "people_skill_level_high",
                    obj.get("people_skill_level_high")
                        .expect("Object must include key 'people_skill_level_high'"),
                ));
                people_skill_level_ultra = LexicalUnitPool::from((
                    "people_skill_level_ultra",
                    obj.get("people_skill_level_ultra")
                        .expect("Object must include key 'people_skill_level_ultra'"),
                ));
                people_trait_ck2 = LexicalUnitPool::from((
                    "people_trait_ck2",
                    obj.get("people_trait_ck2")
                        .expect("Object must include key 'people_trait_ck2'"),
                ));
                people_trait_ck3 = LexicalUnitPool::from((
                    "people_trait_ck3",
                    obj.get("people_trait_ck3")
                        .expect("Object must include key 'people_trait_ck3'"),
                ));
                people_trait_rolls_ck2_mod = LexicalUnitPool::from((
                    "people_trait_rolls_ck2_mod",
                    obj.get("people_trait_rolls_ck2_mod")
                        .expect("Object must include key 'people_trait_rolls_ck2_mod'"),
                ));
                artifact_type = LexicalUnitPool::from((
                    "artifact_type",
                    obj.get("artifact_type")
                        .expect("Object must include key 'artifact_type'"),
                ));
                artifact_adjective = LexicalUnitPool::from((
                    "artifact_adjective",
                    obj.get("artifact_adjective")
                        .expect("Object must include key 'artifact_adjective'"),
                ));
                artifact_material = LexicalUnitPool::from((
                    "artifact_material",
                    obj.get("artifact_material")
                        .expect("Object must include key 'artifact_material'"),
                ));
                artifact_quality = LexicalUnitPool::from((
                    "artifact_quality",
                    obj.get("artifact_quality")
                        .expect("Object must include key 'artifact_quality'"),
                ));
                artifact_art = LexicalUnitPool::from((
                    "artifact_art",
                    obj.get("artifact_art")
                        .expect("Object must include key 'artifact_art'"),
                ));
                artifact_dedication = LexicalUnitPool::from((
                    "artifact_dedication",
                    obj.get("artifact_dedication")
                        .expect("Object must include key 'artifact_dedication'"),
                ));
                scene_actors = LexicalUnitPool::from((
                    "scene_actors",
                    obj.get("scene_actors")
                        .expect("Object must include key 'scene_actors'"),
                ));
                scene_verbs = LexicalUnitPool::from((
                    "scene_verbs",
                    obj.get("scene_verbs")
                        .expect("Object must include key 'scene_verbs'"),
                ));
                scene_objects = LexicalUnitPool::from((
                    "scene_objects",
                    obj.get("scene_objects")
                        .expect("Object must include key 'scene_objects'"),
                ));
                timeline_qualifiers = LexicalUnitPool::from((
                    "timeline_qualifiers",
                    obj.get("timeline_qualifiers")
                        .expect("Object must include key 'timeline_qualifiers'"),
                ));
                ship_prefixes = LexicalUnitPool::from((
                    "ship_prefixes",
                    obj.get("ship_prefixes")
                        .expect("Object must include key 'ship_prefixes'"),
                ));
                ship_long_names = LexicalUnitPool::from((
                    "ship_long_names",
                    obj.get("ship_long_names")
                        .expect("Object must include key 'ship_long_names'"),
                ));
                ship_sizes = LexicalUnitPool::from((
                    "ship_sizes",
                    obj.get("ship_sizes")
                        .expect("Object must include key 'ship_sizes'"),
                ));
                ship_type_ss = LexicalUnitPool::from((
                    "ship_type_ss",
                    obj.get("ship_type_ss")
                        .expect("Object must include key 'ship_type_ss'"),
                ));
                ship_type_xs = LexicalUnitPool::from((
                    "ship_type_xs",
                    obj.get("ship_type_xs")
                        .expect("Object must include key 'ship_type_xs'"),
                ));
                ship_type_s = LexicalUnitPool::from((
                    "ship_type_s",
                    obj.get("ship_type_s")
                        .expect("Object must include key 'ship_type_s'"),
                ));
                ship_type_m = LexicalUnitPool::from((
                    "ship_type_m",
                    obj.get("ship_type_m")
                        .expect("Object must include key 'ship_type_m'"),
                ));
                ship_type_l = LexicalUnitPool::from((
                    "ship_type_l",
                    obj.get("ship_type_l")
                        .expect("Object must include key 'ship_type_l'"),
                ));
                ship_type_xl = LexicalUnitPool::from((
                    "ship_type_xl",
                    obj.get("ship_type_xl")
                        .expect("Object must include key 'ship_type_xl'"),
                ));
                ship_type_xxl = LexicalUnitPool::from((
                    "ship_type_xxl",
                    obj.get("ship_type_xxl")
                        .expect("Object must include key 'ship_type_xxl'"),
                ));
                ship_type_u = LexicalUnitPool::from((
                    "ship_type_u",
                    obj.get("ship_type_u")
                        .expect("Object must include key 'ship_type_u'"),
                ));
                ship_type_xu = LexicalUnitPool::from((
                    "ship_type_xu",
                    obj.get("ship_type_xu")
                        .expect("Object must include key 'ship_type_xu'"),
                ));
                ship_type_t = LexicalUnitPool::from((
                    "ship_type_t",
                    obj.get("ship_type_t")
                        .expect("Object must include key 'ship_type_t'"),
                ));
                ship_fame = LexicalUnitPool::from((
                    "ship_fame",
                    obj.get("ship_fame")
                        .expect("Object must include key 'ship_fame'"),
                ));
                currency_real_names = LexicalUnitPool::from((
                    "currency_real_names",
                    obj.get("currency_real_names")
                        .expect("Object must include key 'currency_real_names'"),
                ));
                currency_real_fractional_names = LexicalUnitPool::from((
                    "currency_real_fractional_names",
                    obj.get("currency_real_fractional_names")
                        .expect("Object must include key 'currency_real_fractional_names'"),
                ));
                currency_endings = LexicalUnitPool::from((
                    "currency_endings",
                    obj.get("currency_endings")
                        .expect("Object must include key 'currency_endings'"),
                ));
                currency_second_word = LexicalUnitPool::from((
                    "currency_second_word",
                    obj.get("currency_second_word")
                        .expect("Object must include key 'currency_second_word'"),
                ));
                currency_metals = LexicalUnitPool::from((
                    "currency_metals",
                    obj.get("currency_metals")
                        .expect("Object must include key 'currency_metals'"),
                ));
                currency_icon_inscription = LexicalUnitPool::from((
                    "currency_icon_inscription",
                    obj.get("currency_icon_inscription")
                        .expect("Object must include key 'currency_icon_inscription'"),
                ));
                currency_icon_figure = LexicalUnitPool::from((
                    "currency_icon_figure",
                    obj.get("currency_icon_figure")
                        .expect("Object must include key 'currency_icon_figure'"),
                ));
                currency_icon_partial = LexicalUnitPool::from((
                    "currency_icon_partial",
                    obj.get("currency_icon_partial")
                        .expect("Object must include key 'currency_icon_partial'"),
                ));
                currency_icon_incuse = LexicalUnitPool::from((
                    "currency_icon_incuse",
                    obj.get("currency_icon_incuse")
                        .expect("Object must include key 'currency_icon_incuse'"),
                ));
                currency_icon_insignia = LexicalUnitPool::from((
                    "currency_icon_insignia",
                    obj.get("currency_icon_insignia")
                        .expect("Object must include key 'currency_icon_insignia'"),
                ));
                currency_icon_mythical_creature = LexicalUnitPool::from((
                    "currency_icon_mythical_creature",
                    obj.get("currency_icon_mythical_creature")
                        .expect("Object must include key 'currency_icon_mythical_creature'"),
                ));
                currency_icon_animal = LexicalUnitPool::from((
                    "currency_icon_animal",
                    obj.get("currency_icon_animal")
                        .expect("Object must include key 'currency_icon_animal'"),
                ));
                currency_icon_plant = LexicalUnitPool::from((
                    "currency_icon_plant",
                    obj.get("currency_icon_plant")
                        .expect("Object must include key 'currency_icon_plant'"),
                ));
                metals_list = LexicalUnitPool::from((
                    "metals_list",
                    obj.get("metals_list")
                        .expect("Object must include key 'metals_list'"),
                ));
                metals_alloy_list = LexicalUnitPool::from((
                    "metals_alloy_list",
                    obj.get("metals_alloy_list")
                        .expect("Object must include key 'metals_alloy_list'"),
                ));
                government_name0 = LexicalUnitPool::from((
                    "government_name0",
                    obj.get("government_name0")
                        .expect("Object must include key 'government_name0'"),
                ));
                government_name1 = LexicalUnitPool::from((
                    "government_name1",
                    obj.get("government_name1")
                        .expect("Object must include key 'government_name1'"),
                ));
                government_name_monarchy = LexicalUnitPool::from((
                    "government_name_monarchy",
                    obj.get("government_name_monarchy")
                        .expect("Object must include key 'government_name_monarchy'"),
                ));
                rep_or_state_list = LexicalUnitPool::from((
                    "rep_or_state_list",
                    obj.get("rep_or_state_list")
                        .expect("Object must include key 'rep_or_state_list'"),
                ));
                story_empire_interactions = LexicalUnitPool::from((
                    "story_empire_interactions",
                    obj.get("story_empire_interactions")
                        .expect("Object must include key 'story_empire_interactions'"),
                ));
                story_government_type_adj_0 = LexicalUnitPool::from((
                    "story_government_type_adj_0",
                    obj.get("story_government_type_adj_0")
                        .expect("Object must include key 'story_government_type_adj_0'"),
                ));
                story_government_type_adj_01 = LexicalUnitPool::from((
                    "story_government_type_adj_01",
                    obj.get("story_government_type_adj_01")
                        .expect("Object must include key 'story_government_type_adj_01'"),
                ));
                story_government_type_adj_1 = LexicalUnitPool::from((
                    "story_government_type_adj_1",
                    obj.get("story_government_type_adj_1")
                        .expect("Object must include key 'story_government_type_adj_1'"),
                ));
                story_government_type_sec_0 = LexicalUnitPool::from((
                    "story_government_type_sec_0",
                    obj.get("story_government_type_sec_0")
                        .expect("Object must include key 'story_government_type_sec_0'"),
                ));
                story_government_type_sec_01 = LexicalUnitPool::from((
                    "story_government_type_sec_01",
                    obj.get("story_government_type_sec_01")
                        .expect("Object must include key 'story_government_type_sec_01'"),
                ));
                story_government_type_sec_1 = LexicalUnitPool::from((
                    "story_government_type_sec_1",
                    obj.get("story_government_type_sec_1")
                        .expect("Object must include key 'story_government_type_sec_1'"),
                ));
                story_government_leader_title_0_reli = LexicalUnitPool::from((
                    "story_government_leader_title_0_reli",
                    obj.get("story_government_leader_title_0_reli")
                        .expect("Object must include key 'story_government_leader_title_0_reli'"),
                ));
                story_government_leader_title_0_cor = LexicalUnitPool::from((
                    "story_government_leader_title_0_cor",
                    obj.get("story_government_leader_title_0_cor")
                        .expect("Object must include key 'story_government_leader_title_0_cor'"),
                ));
                story_government_leader_title_0_mon = LexicalUnitPool::from((
                    "story_government_leader_title_0_mon",
                    obj.get("story_government_leader_title_0_mon")
                        .expect("Object must include key 'story_government_leader_title_0_mon'"),
                ));
                story_government_leader_title_01 = LexicalUnitPool::from((
                    "story_government_leader_title_01",
                    obj.get("story_government_leader_title_01")
                        .expect("Object must include key 'story_government_leader_title_01'"),
                ));
                story_government_leader_title_1 = LexicalUnitPool::from((
                    "story_government_leader_title_1",
                    obj.get("story_government_leader_title_1")
                        .expect("Object must include key 'story_government_leader_title_1'"),
                ));
                person_social_gender = LexicalUnitPool::from((
                    "person_social_gender",
                    obj.get("person_social_gender")
                        .expect("Object must include key 'person_social_gender'"),
                ));
            }
            Err(error) => return Err(Error::other(error)),
        }
        let people_skill_level_all = LexicalUnitPool::new("people_skill_level")
            .extend(people_skill_level_low.clone())
            .extend(people_skill_level_medium.clone())
            .extend(people_skill_level_high.clone())
            .extend(people_skill_level_ultra.clone());
        Ok(LexicalUnitLagoon {
            abc,
            place_relatives,
            place_noun,
            place_top_eng_nouns,
            place_object,
            place_single,
            general_comp0,
            general_comp1,
            people_first_name,
            people_last_name,
            people_nickname,
            people_title,
            people_skill,
            people_skill_level_all,
            people_skill_level_low,
            people_skill_level_medium,
            people_skill_level_high,
            people_skill_level_ultra,
            people_trait_ck2,
            people_trait_ck3,
            people_trait_rolls_ck2_mod,
            artifact_type,
            artifact_adjective,
            artifact_material,
            artifact_quality,
            artifact_art,
            artifact_dedication,
            scene_actors,
            scene_verbs,
            scene_objects,
            timeline_qualifiers,
            ship_prefixes,
            ship_long_names,
            ship_sizes,
            ship_type_ss,
            ship_type_xs,
            ship_type_s,
            ship_type_m,
            ship_type_l,
            ship_type_xl,
            ship_type_xxl,
            ship_type_u,
            ship_type_xu,
            ship_type_t,
            ship_fame,
            currency_real_names,
            currency_real_fractional_names,
            currency_endings,
            currency_second_word,
            currency_metals,
            currency_icon_inscription,
            currency_icon_figure,
            currency_icon_partial,
            currency_icon_incuse,
            currency_icon_insignia,
            currency_icon_mythical_creature,
            currency_icon_animal,
            currency_icon_plant,
            metals_list,
            metals_alloys_list: metals_alloy_list,
            government_name0,
            government_name1,
            government_name_monarchy,
            rep_or_state_list,
            story_empire_interactions,
            story_government_type_adj_0,
            story_government_type_adj_01,
            story_government_type_adj_1,
            story_government_type_sec_0,
            story_government_type_sec_01,
            story_government_type_sec_1,
            story_government_leader_title_0_reli,
            story_government_leader_title_0_cor,
            story_government_leader_title_0_mon,
            story_government_leader_title_01,
            story_government_leader_title_1,
            person_social_gender,
        })
    }
}
