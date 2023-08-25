use prost_build::Config;

fn main() {
    let mut config: Config = Config::new();
    config.out_dir("src/generated/");
    config
        .compile_protos(&["src/proto/order_requests.proto"], &["src/proto"])
        .expect("Could not compile orders.proto");
    config
        .compile_protos(&["src/proto/proto_decimal.proto"], &["src/proto"])
        .expect("Could not compile proto_decimal.proto");
    config
        .compile_protos(&["src/proto/request_response.proto"], &["src/proto"])
        .expect("Could not compile request_response.proto")
}
