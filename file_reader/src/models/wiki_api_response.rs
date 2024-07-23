use rubit_api_db::dictionary_info::russian::RussianNoun;

struct WikiApiResponse {
    raw: String,
}

impl WikiApiResponse {
    pub fn parse_noun_table(&self) -> rubit_api_db::dictionary_info::russian::RussianNoun {
        RussianNoun {
            dictionary_form: todo!(),
            accented: todo!(),
            ipa: todo!(),
            nom_sing: todo!(),
            nom_plur: todo!(),
            acc_sing: todo!(),
            acc_plur: todo!(),
            gen_sing: todo!(),
            gen_plur: todo!(),
            dat_sing: todo!(),
            dat_plur: todo!(),
            ins_sing: todo!(),
            ins_plur: todo!(),
            pre_sing: todo!(),
            pre_plur: todo!(),
        }
    }
}
