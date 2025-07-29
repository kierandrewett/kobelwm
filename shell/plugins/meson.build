plugins = [
    'autostart'
]

all_include_dirs = [wayfire_api_inc, wayfire_conf_inc, plugins_common_inc, vswitch_inc, ipc_include_dirs]

all_deps = [wlroots, pixman, wfconfig, wftouch, cairo, pango, pangocairo, json, plugin_pch_dep]

extra_libs = {
    'move': [move_drag_interface],
    'expo': [workspace_wall, move_drag_interface],
    'vswipe': [workspace_wall],
}

devenv = environment()
foreach plugin : plugins
    devenv.append('WAYFIRE_PLUGIN_PATH', meson.current_build_dir() + '/' + plugin)

    shared_module(plugin, plugin + '.cpp',
        include_directories: all_include_dirs,
        dependencies: all_deps,
        link_with: extra_libs.get(plugin, []),
        install: true,
        install_dir: conf_data.get('PLUGIN_PATH'))

    install_data(plugin + '.xml', install_dir: conf_data.get('PLUGIN_XML_DIR'))
endforeach
meson.add_devenv(devenv)
