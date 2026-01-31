// User domain module - core business rules for user management

pub mod entity;
pub mod repository;
pub mod value_objects;

pub use entity::{PublicUserProfile, User, UserProfile, UserToken};
pub use repository::{UserProfileRepository, UserRepository, TokenRepository};
pub use value_objects::{
    EditableUser, GoogleUserInfo, LoginResponse, NewUser, NewUserProfile, UpdateAvatarRequest,
    UpdateNotificationPreferences, UpdatePrivacySettings, UpdateUserPreferences, UpdateUserProfile,
};
