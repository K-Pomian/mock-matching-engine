use prost_build::Config;

fn main() {
    let mut config: Config = Config::new();
    config.out_dir("src/generated/");
    config
        .compile_protos(&["src/proto/order_requests.proto"], &["src/proto"])
        .expect("Could not compile orders.proto");
}
