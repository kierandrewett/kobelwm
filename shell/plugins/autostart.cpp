#include <wayfire/plugin.hpp>
#include <wayfire/core.hpp>
#include <wayfire/util/log.hpp>
#include <wayfire/option-wrapper.hpp>
#include <wayfire/config/config-manager.hpp>
#include <config.h>

class kobel_autostart : public wf::plugin_interface_t
{
    wf::option_wrapper_t<bool> autostart_kobel_shell{"autostart/autostart_kobel_shell"};
    wf::option_wrapper_t<wf::config::compound_list_t<std::string>>
    autostart_entries{"autostart/autostart"};

    public:
        void init() override
        {
            /* Run only once, at startup */
            auto section = wf::get_core().config->get_section("autostart");

            bool shell_manually_started = false;

            for (const auto& [name, command] : autostart_entries.value())
            {
                // Because we accept any option names, we should ignore regular
                // options
                if (name == "autostart_kobel_shell")
                {
                    continue;
                }

                wf::get_core().run(command);
                if (command.find("kobel-shell") != std::string::npos)
                {
                    shell_manually_started = true;
                }
            }

            if (autostart_kobel_shell && !shell_manually_started)
            {
                wf::get_core().run("kobel-shell");
                wf::get_core().run("wf-background");

            }
        }

        bool is_unloadable() override
        {
            return false;
        }
};

DECLARE_WAYFIRE_PLUGIN(kobel_autostart);
