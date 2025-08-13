#[derive(Debug)]
pub struct user {
    pub id: i64,
    pub status: i32,
    pub company: i64,
    pub name: Option<String>,
    pub telp: Option<String>,
    pub password: String,
    pub is_login: bool,
}

impl user {
    pub fn new(
        id: i64,
        status: i32,
        company: i64,
        name: Option<String>,
        password: String,
        telp: Option<String>,
        is_login: bool,
    ) -> Self {
        Self {
            id,
            status,
            company,
            name,
            password,
            telp,
            is_login,
        }
    }
}
