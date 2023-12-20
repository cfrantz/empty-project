load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")
load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

def google_deps():
    protobuf_deps()
    rules_proto_dependencies()
    rules_proto_toolchains()
