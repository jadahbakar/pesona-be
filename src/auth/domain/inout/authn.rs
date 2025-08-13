use validator::Validate;
// -----------------------
//  Login
// -----------------------
#[derive(Debug, Validate)]
pub struct LoginInput {
    #[validate(email(message = "must be a valid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "minimal 8 characters length"))]
    pub password: String,
}

#[derive(Debug, Validate)]
pub struct LoginOutput {
    pub access_token: String,
}
