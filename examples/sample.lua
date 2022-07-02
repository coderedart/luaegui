My_plugin = {}
My_plugin.window_options = {
    title = "my lua window",
    open = true
}
My_plugin.button_options = {
    widget_type = "button",
    text = "my lua button",
}
On_gui = function(ctx)
    ctx:new_window(
        My_plugin.window_options,
        function(ui)
            local response_vec, inner, inner_inner = ui:scope(
                function(ui)
                    return ui:scope(
                        function(ui)
                            return ui:add(My_plugin.button_options)
                        end
                    )
                end
            )
            -- local response, inner = response_vec[1], response_vec[2]
            -- local inner, inner_inner = inner[1], inner[2]
            -- if response:clicked() then
            --     print("outer")
            -- end
            if inner:clicked() then
                print("inner")
            end
            if inner_inner:clicked() then
                print("inner inner")
                print(Egui.color32.default().r)
            end
        end
    );
end
