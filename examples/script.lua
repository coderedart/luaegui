my_data = {
    text = "my text"
}
function show_fn(ui)
    ui:label(my_data.text);
    ui:text_edit_singleline(my_data);
    if ui:button("cute button"):clicked() then
        print("cute button pressed. printing my_data.text");
        print(my_data.text);
    end
    
end

_G.gui_run = function (ctx)
    local top_panel = egui.top_bottom_panel.top("top panel");
    top_panel:show(ctx, 
        function (ui) 
            ui:menu_button("my menu",
                function (ui) 
                    ui:label("empty :(");
                end
            );
        end
    );
    local new_window = egui.window.new("my lua window");
    new_window:show(ctx, show_fn);
end
