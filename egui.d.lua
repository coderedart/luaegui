declare class Context
    function clone(self): Context
end
declare class Response
    function clicked(self): boolean
end
declare class Ui
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