My_plugin = {}
My_plugin.window_options = {
    title = "my lua window",
    open = true
}
My_plugin.button_options = {
    widget_type = "button",
    text = "my lua button",
}
My_plugin.background = Egui.color32.default()
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
            ui:columns(3, function(columns)
                columns[1]:label("column 1")
                local response, background = columns[1]:color_edit_button_srgba(My_plugin.background)
                My_plugin.background = background
                columns[2]:label("column 2")
                columns[3]:label("column 3")

            end)
            if inner:clicked() then
                print("inner")
            end
            if inner_inner:clicked() then
                print(ui:next_widget_position().x)
            end
        end
    );
end
