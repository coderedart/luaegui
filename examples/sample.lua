My_plugin = {}
My_plugin.window_options = {
    title = "my lua window",
    open = true,
    ui_function = function(ui)

        ui:columns(3, function(columns)
            columns[1]:label("column 1")
            local response, background = columns[1]:color_edit_button_srgba(My_plugin.background)
            My_plugin.background = background
            columns[2]:label("column 2")
            columns[3]:label("column 3")

        end)

    end,
    window_configuration_function = function(ctx, window)
        window:open(My_plugin.window_options.open)
        local open, window_response, inner_response = window:show(ctx, My_plugin.window_options.ui_function)
        My_plugin.open = open
    end
}
My_plugin.button_options = {
    widget_type = "button",
    text = "my lua button",
}
My_plugin.background = Egui.color32.default()
On_gui = function(ctx)
    Egui.window.new(
        My_plugin.window_options.title,
        ctx,
        My_plugin.window_options.window_configuration_function
    );

end
