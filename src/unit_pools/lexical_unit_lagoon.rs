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
