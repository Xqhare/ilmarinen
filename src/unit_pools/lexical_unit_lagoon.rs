use crate::lexical_unit_pool::LexicalUnitPool;


#[derive(Debug)]
struct LexicalUnitLagoon {
    abc: LexicalUnitPool,

    place_relatives: LexicalUnitPool,
    place_noun: LexicalUnitPool,
    /// The most used english nouns
    place_top_eng_nouns: LexicalUnitPool,
    place_object: LexicalUnitPool,
    place_single: LexicalUnitPool,

    general_comp0: LexicalUnitPool,
    general_comp1: LexicalUnitPool,

    people_first_name: LexicalUnitPool,
    people_last_name: LexicalUnitPool,
    people_nickname: LexicalUnitPool,
    people_title: LexicalUnitPool,
    people_skill: LexicalUnitPool,
    people_skill_level: LexicalUnitPool,
    people_trait_ck2: LexicalUnitPool,
    people_trait_ck3: LexicalUnitPool,

    artifact_type: LexicalUnitPool,
    artifact_adjective: LexicalUnitPool,
    artifact_material: LexicalUnitPool,
    artifact_quality: LexicalUnitPool,
    artifact_dedication: LexicalUnitPool,

    scene_actors: LexicalUnitPool,

    timeline_qualifiers: LexicalUnitPool,

    ship_prefixes: LexicalUnitPool,
    ship_long_names: LexicalUnitPool,
    ship_sizes: LexicalUnitPool,
    ship_type_ss: LexicalUnitPool,
    ship_type_xs: LexicalUnitPool,
    ship_type_s: LexicalUnitPool,
    ship_type_m: LexicalUnitPool,
    ship_type_l: LexicalUnitPool,
    ship_type_xl: LexicalUnitPool,
    ship_type_xxl: LexicalUnitPool,
    ship_type_u: LexicalUnitPool,
    ship_type_xu: LexicalUnitPool,
    ship_type_t: LexicalUnitPool,
    ship_fame: LexicalUnitPool,

    currency_real_names: LexicalUnitPool,
    currency_real_fractional_names: LexicalUnitPool,
    currency_coins_denomination: LexicalUnitPool,
    currency_non_decimal_base_120: LexicalUnitPool,
    currency_endings: LexicalUnitPool,
    currency_second_word: LexicalUnitPool,
    currency_metals: LexicalUnitPool,
    currency_icon_inscription: LexicalUnitPool,
    currency_icon_figure: LexicalUnitPool,
    currency_icon_partial: LexicalUnitPool,
    currency_icon_incuse: LexicalUnitPool,
    currency_icon_mythical_creature: LexicalUnitPool,
    currency_icon_animal: LexicalUnitPool,
    currency_icon_plant: LexicalUnitPool,

    metals_list: LexicalUnitPool,
    metals_alloys_list: LexicalUnitPool,

    government_name0: LexicalUnitPool,
    government_name1: LexicalUnitPool,
    government_name_monarchy: LexicalUnitPool,
    rep_or_state_list: LexicalUnitPool,

    story_empire_interactions: LexicalUnitPool,
    story_government_type_adj_0: LexicalUnitPool,
    story_government_type_adj_01: LexicalUnitPool,
    story_government_type_adj_1: LexicalUnitPool,
    story_government_type_sec_0: LexicalUnitPool,
    story_government_type_sec_01: LexicalUnitPool,
    story_government_type_sec_1: LexicalUnitPool,
    story_government_leader_title_0_reli: LexicalUnitPool,
    story_government_leader_title_0_cor: LexicalUnitPool,
    story_government_leader_title_0_mon: LexicalUnitPool,
    story_government_leader_title_01: LexicalUnitPool,
    story_government_leader_title_1: LexicalUnitPool,
}
