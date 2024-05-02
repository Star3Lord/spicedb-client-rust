mod proto {
    tonic::include_proto!("mod");
}

#[cfg(feature = "v0")]
pub mod v0 {
    pub use super::proto::authzed::api::v0::*;
}

#[cfg(feature = "v1")]
pub mod v1 {
    pub use super::proto::authzed::api::v1::*;
}

#[cfg(feature = "v1alpha1")]
pub mod v1alpha1 {
    pub use super::proto::authzed::api::v1alpha1::*;
}
