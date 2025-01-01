pub mod helloworld;
pub mod user;

// 明确导出而不是使用 glob 导入
pub use helloworld::proto as helloworld_proto;
pub use helloworld::HelloWorldGrpcServiceImpl;
pub use user::proto as user_proto;
pub use user::UserGrpcServiceImpl;
