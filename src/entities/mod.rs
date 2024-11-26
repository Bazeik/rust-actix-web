pub mod user;

pub mod prelude {
    pub use super::user::Entity as User;
    pub use super::user::ActiveModel as UserActiveModel;
}