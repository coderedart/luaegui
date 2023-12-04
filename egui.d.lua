declare class Context
    function clone(self): Context
end
declare class Response
    function clicked(self): boolean
end
declare class InnerResponse
    response: Response
    inner: any
end

declare class Ui
    function add_enabled_ui(self, enabled: boolean, uifn: (Ui) -> any): InnerResponse
    function add_space(self, space: number): ()
    function add_visible_ui(self, visible: boolean, uifn: (Ui) -> any): InnerResponse
    
    function label(self, text: string): Response
    function text_edit_singleline(self, text: string): Response
    function button(self, text: string): Response

end
declare class TopBottomPanel
    function show(self, ctx: Context, uifn: (Ui) -> any): any
end
declare class Window
    function show(self, ctx: Context, uifn: (Ui) -> any): any
end
declare egui: {
    top_bottom_panel: {
        top: (string) -> TopBottomPanel,
        bottom: (string) -> TopBottomPanel
    },
    window: {
        new: (string) -> Window
    }
}