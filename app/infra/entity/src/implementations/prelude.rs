use sea_orm::DbErr;

pub trait DbErrUtils {
    fn get_db_constrint_err(&self) -> Option<String>;
}

impl DbErrUtils for DbErr {
    fn get_db_constrint_err(&self) -> Option<String> {
        match self {
            sea_orm::error::DbErr::Query(runtime_err) => match runtime_err {
                sea_orm::error::RuntimeErr::SqlxError(sqlx_err) => match sqlx_err {
                    sqlx::error::Error::Database(db_err) => {
                        db_err.constraint().map(|s| s.to_owned())
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        }
    }
}
