pub mod grpc;
pub mod helloworld;

// 明确导出而不是使用 glob 导入
pub use grpc::proto as service_proto;
pub use grpc::YourGrpcServiceImpl;
pub use helloworld::proto as helloworld_proto;
pub use helloworld::HelloWorldGrpcServiceImpl;
