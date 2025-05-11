const std = @import("std");
const build_crab = @import("build_crab");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    _ = b.addModule("rusterizer", .{
        .root_source_file = b.path("binding/rusterizer.zig"),
        .target = target,
        .optimize = optimize,
    });

    const as_artifacts = build_crab.addCargoBuild(
        b,
        .{
            .manifest_path = b.path("Cargo.toml"),
            // You can pass additional arguments to Cargo
            .cargo_args = &.{
                "--release",
                "--quiet",
            },
        },
        .{
            // Set to .Debug to see debug logs,
            // defaults to the same optimization level as your package.
            .optimize = .ReleaseSafe,
            .target = target,
        },
    );

    // WARNING: This is really a hack, check https://zig.news/edyu/zig-package-manager-wtf-is-zon-2-0110-update-1jo3 for details.
    _ = b.addModule("librusterizer.so", .{
        .root_source_file = as_artifacts.path(b, "librusterizer.so"),
    });

    switch (target.result.os.tag) {
        .linux, .macos => {
            _ = b.addModule("librusterizer.so", .{
                .root_source_file = as_artifacts.path(b, "librusterizer.so"),
            });
        },
        .windows => {
            _ = b.addModule("librusterizer.dll", .{
                .root_source_file = as_artifacts.path(b, "librusterizer.dll"),
            });
            _ = b.addModule("librusterizer.lib", .{
                .root_source_file = as_artifacts.path(b, "librusterizer.lib"),
            });
        },
        else => {},
    }
}
