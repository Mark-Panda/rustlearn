// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tonic_build::compile_protos("src/proto/service.proto")?;
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=src/proto");

    tonic_build::configure()
        // .build_client(true) // 是否生成客户端代码
        // .build_server(true) // 是否生成服务端代码
        // .type_attribute(".", "#[derive(Hash)]") // 为所有消息添加派生宏
        // .field_attribute(".", "#[allow(unused)]") // 为所有字段添加属性
        // .out_dir("src/generated") // 指定输出目录
        .compile_protos(
            &["src/proto/helloworld.proto", "src/proto/service.proto"],
            &["src/proto"],
        )?;

    Ok(())
}
