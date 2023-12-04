my_data = {
    text = "my text"
}
-- a function to run inside the window
function window_ui(ui)
    ui:label(my_data.text);
    ui:text_edit_singleline(my_data);
    if ui:button("cute button"):clicked() then
        print("cute button pressed.");
    end
end
-- will be called every frame with egui Context as arg
_G.gui_run = function (ctx)
    local new_window = egui.window.new("my lua window");
    new_window:show(ctx, window_ui);
end
