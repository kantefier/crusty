fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[ &[WE_PROTO_SRC_LOCATION, "/messagebroker/messagebroker_blockchain_events_service.proto"].concat() ],
            &[ &WE_PROTO_SRC_LOCATION.to_owned() ]
        ).unwrap();
}

const WE_PROTO_SRC_LOCATION: &str = "we_proto_src";