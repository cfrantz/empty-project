load(
    "@bazel_tools//tools/cpp:toolchain_utils.bzl",
    "CPP_TOOLCHAIN_TYPE",
    "find_cpp_toolchain",
)

def _extract_files(ctx):
    f = ctx.attr.src[DefaultInfo]
    fs = f.files.to_list() + f.default_runfiles.files.to_list()
    #output = []
    #for f in fs:
    #    for m in ctx.attr.match:
    #        if f.short_path.endswith(m):
    #            output.append(f)
    output = [f for f in fs for m in ctx.attr.match if f.short_path.endswith(m)]
    return DefaultInfo(files=depset([output]))

extract_files = rule(
    implementation = _extract_files,
    attrs = {
        "src": attr.label(doc="Source rule", providers=[DefaultInfo]),
        "match": attr.string_list(doc="File path suffixes to match")
    }
)

def __pyext_cc_library(name, src, library):
    ext = name + "_extract"
    extract_files(
        name = ext,
        src = src,
        match = [library],
    )
    native.cc_import(
        name = name,
        shared_library = ":" + ext,
    )

def _pyext_cc_library(ctx):
    cc = find_cpp_toolchain(ctx)

    src = ctx.attr.src[DefaultInfo]
    srcs = src.files.to_list() + src.default_runfiles.files.to_list()
    lib = [f for f in srcs if f.short_path.endswith(ctx.attr.library)]
    if not lib:
        fail("Could not match", ctx.attr.library, "in", ctx.attr.src)
    if len(lib) > 1:
        fail("Expected to find exactly one match, but found", len(lib))
    lib = lib[0]

    #compilation_info = cc_common.create_compilation_context(
    #    headers = depset([]),
    #    system_includes = depset([]),
    #    includes = depset([]),
    #    quote_includes = depset([]),
    #)
    features = cc_common.configure_features(
        ctx = ctx,
        cc_toolchain = cc,
        requested_features = ctx.features + [
            "dynamic_linking_mode",
        ],
        unsupported_features = ctx.disabled_features + [
            "fdo_instrument",
            "fdo_optimize",
            "layering_check",
            "module_maps",
            "thin_lto",
        ],
    )
    linking_info = cc_common.create_linking_context(
        linker_inputs = depset(direct = [
            cc_common.create_linker_input(
                owner = ctx.label,
                libraries = depset(direct = [cc_common.create_library_to_link(
                    actions = ctx.actions,
                    feature_configuration = features,
                    cc_toolchain = cc,
                    dynamic_library = lib,
                )]),
            ),
        ]),
    )
    cc_info = [CcInfo(
        #compilation_context = compilation_info,
        compilation_context = None,
        linking_context = linking_info,
    )]
    print(cc_info)
    return cc_info

pyext_cc_library = rule(
    implementation = _pyext_cc_library,
    attrs = {
        "src": attr.label(doc="Source rule", providers=[DefaultInfo]),
        "library": attr.string(doc="File path suffix to match"),
        "_cc_toolchain": attr.label(
             default = Label("@bazel_tools//tools/cpp:toolchain"),
        ),
    },
    fragments = ["cpp"],
    toolchains = [
        config_common.toolchain_type(CPP_TOOLCHAIN_TYPE, mandatory = False),
    ],
)
