use super::index_vec::IndexVec;

pub struct TSQLValue(String);

pub trait ToTSQLValue {
    fn to(&self) -> TSQLValue;
    fn to_id(&self) -> TSQLValue {
        TSQLValue(String::new())
    }
}

impl ToTSQLValue for String {
    fn to(&self) -> TSQLValue {
        TSQLValue(format!("'{}'", str::replace(self, "'", "\\'")))
    }
}

impl ToTSQLValue for i32 {
    fn to(&self) -> TSQLValue {
        TSQLValue(self.to_string())
    }
    fn to_id(&self) -> TSQLValue {
        TSQLValue(format!("id{}", self))
    }
}

impl ToTSQLValue for chrono::NaiveDateTime {
    fn to(&self) -> TSQLValue {
        TSQLValue(self.format("%FP%T").to_string())
    }
}

impl ToTSQLValue for dyn ToString {
    fn to(&self) -> TSQLValue {
        self.to_string().to()
    }
}

pub trait ToTSQL {
    fn to_header() -> &'static str;  // Stonker
    fn to_columns() -> Vec<&'static str>;  // [ id ] [ name ] [ password ]
    fn to_data(&self) -> Vec<TSQLValue>; // frank1 "Frank" "knarf"
    fn convert(data: IndexVec<Self>) -> String where Self: Sized {
        format!("{{ {} }}\n{}\n{}\n", Self::to_header(),
            Self::to_columns().into_iter().map(|column| format!("[ {} ] ", column)).collect::<String>(),
            data.into_iter().map(|data|
                format!("{}\n", Self::to_data(&data).into_iter()
                    .map(|column| format!("{} ", column.0)).collect::<String>()),
            ).collect::<String>())
    }
}
