use tealr::mlu::mlua::*;
use tealr::new_type;

use crate::*;

static mut WINDOW_OPEN: bool = true;
pub type Window<'a> = Wrapper<Option<egui::Window<'a>>>;

impl<'lua> tealr::TypeName for Window<'lua> {
    fn get_type_parts() -> std::borrow::Cow<'static, [tealr::NamePart]> {
        tealr::new_type!(Window)
    }
}

impl<'a> TealData for Window<'a> {
    fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document(r#" This is the function you use to create a new window.
        This function will need three arguments;
        1. Title of the window. 
        2. The egui Context that is provided to the OnGui function
        3. A function callback. this function will be called with two arguments.
            1. The Context which was provided to us. this needs to be passed the "show" function of the Window. 
            2. The temporary Window object. just like Ui, you can call functions on it. but you can only call "show" function once on this. 
            Read the show function docs for other details.
            
        Windows are temporary much like Ui objects. so, you cannot store them.  
        "#);
        methods.add_function("new", |lua, args: (IntoWidgetText, Context, Function)| {
            let window = egui::Window::new(args.0);
            lua.scope(|scope| {
                let data = scope.create_nonstatic_userdata(Wrapper(Some(window)))?;
                args.2.call::<_, ()>((args.1, data))?;
                Ok(())
            })?;
            Ok(())
        });
        methods.add_method_mut("id", |_, self_ref, a0: Id| {
            self_ref.0 = self_ref.0.take().map(|w| w.id(a0.into()));
            Ok(())
        });

        methods.document("whether window should be open. when you call the show function, it will return the new status of the window.");
        methods.add_method_mut("open", |_, self_ref, a0: bool| {
            self_ref.0 = self_ref.0.take().map(|w| unsafe {
                WINDOW_OPEN = a0;
                w.open(&mut WINDOW_OPEN)
            });
            Ok(())
        });
        methods.add_method_mut("enabled", |_, self_ref, a0: bool| {
            self_ref.0 = self_ref.0.take().map(|w| w.enabled(a0));
            Ok(())
        });
        // methods.add_method_mut("mutate", |_, self_ref, a0: Function| {
        //     self_ref.0 = self_ref.0.take().map(|w| {
        //         w.id(a0.into())
        //     });
        //     Ok(())
        // });
        methods.document("the function callback will be given a Resize object. it can use any resizing methods it wants, BUT it must return the resize object at the end of the function callback");
        methods.add_method_mut("resize", |_, self_ref, a0: Function| {
            self_ref.0 = self_ref.0.take().map(|w| {
                w.resize(|resize| {
                    let resize: Resize = a0
                        .call(Resize::from(resize))
                        .expect("resize callback error. must return resize");
                    resize.into()
                })
            });
            Ok(())
        });
        methods.add_method_mut("frame", |_, self_ref, a0: Frame| {
            self_ref.0 = self_ref.0.take().map(|w| w.frame(a0.into()));
            Ok(())
        });
        methods.add_method_mut("min_width", |_, self_ref, a0: f32| {
            self_ref.0 = self_ref.0.take().map(|w| w.min_width(a0));
            Ok(())
        });
        methods.add_method_mut("min_height", |_, self_ref, a0: f32| {
            self_ref.0 = self_ref.0.take().map(|w| w.min_height(a0));
            Ok(())
        });
        methods.add_method_mut("current_pos", |_, self_ref, a0: Pos2| {
            self_ref.0 = self_ref.0.take().map(|w| w.current_pos(a0));
            Ok(())
        });
        methods.add_method_mut("default_pos", |_, self_ref, a0: Pos2| {
            self_ref.0 = self_ref.0.take().map(|w| w.default_pos(a0));
            Ok(())
        });
        methods.add_method_mut("anchor", |_, self_ref, args: (Align2, Vec2)| {
            self_ref.0 = self_ref.0.take().map(|w| w.anchor(args.0.into(), args.1));
            Ok(())
        });
        methods.add_method_mut("default_size", |_, self_ref, a0: Vec2| {
            self_ref.0 = self_ref.0.take().map(|w| w.default_size(a0));
            Ok(())
        });
        methods.add_method_mut("default_width", |_, self_ref, a0: f32| {
            self_ref.0 = self_ref.0.take().map(|w| w.default_width(a0));
            Ok(())
        });
        methods.add_method_mut("default_height", |_, self_ref, a0: f32| {
            self_ref.0 = self_ref.0.take().map(|w| w.default_height(a0));
            Ok(())
        });
        methods.add_method_mut("default_rect", |_, self_ref, a0: Rect| {
            self_ref.0 = self_ref.0.take().map(|w| w.default_rect(a0.into()));
            Ok(())
        });
        methods.add_method_mut("resizable", |_, self_ref, a0: bool| {
            self_ref.0 = self_ref.0.take().map(|w| w.resizable(a0));
            Ok(())
        });
        methods.add_method_mut("collapsible", |_, self_ref, a0: bool| {
            self_ref.0 = self_ref.0.take().map(|w| w.collapsible(a0));
            Ok(())
        });
        methods.add_method_mut("title_bar", |_, self_ref, a0: bool| {
            self_ref.0 = self_ref.0.take().map(|w| w.title_bar(a0));
            Ok(())
        });
        methods.add_method_mut("auto_sized", |_, self_ref, ()| {
            self_ref.0 = self_ref.0.take().map(|w| w.auto_sized());
            Ok(())
        });
        methods.add_method_mut("scroll2", |_, self_ref, a0: [bool; 2]| {
            self_ref.0 = self_ref.0.take().map(|w| w.scroll2(a0));
            Ok(())
        });
        methods.add_method_mut("hscroll", |_, self_ref, a0: bool| {
            self_ref.0 = self_ref.0.take().map(|w| w.hscroll(a0));
            Ok(())
        });
        methods.add_method_mut("vscroll", |_, self_ref, a0: bool| {
            self_ref.0 = self_ref.0.take().map(|w| w.vscroll(a0));
            Ok(())
        });
        methods.add_method_mut("drag_bounds", |_, self_ref, a0: Rect| {
            self_ref.0 = self_ref.0.take().map(|w| w.drag_bounds(a0.into()));
            Ok(())
        });

        methods.document(r#"
        This function will consume the window and using the settings from that, display the window. 
        it will take the Context as the first argument and the Ui callback function. that callback will be given the temporary Ui Object.
        The callback WILL NOT BE called if the window is collapsed as there's no reason to show anything.
        the return values can vary based on the status of the Window and will be in the following order.
        1. bool. this will always be the first return value, and represents the status of the Window.
        2. if the window is not OPEN (closed completely). then, this will be a nil value. if the window is open, this will contain the response form the window.
        3. if the window is collapsed, this will be null because Ui callback is not run. otherwise, the return values will be from the result of running Ui callback. 
        "#);
        methods.add_method_mut("show", |lua, self_ref, args: (Context, Function)| {
            let window = self_ref
                .0
                .take()
                .expect("show function called on a window that doesn't exist");
            let result = window.show(&args.0, |ui| {
                let result = lua_registry_scoped_ui_extract!(lua, ui, |ui| { args.1.call(ui) });
                result
            });
            let mut multi_value = match result {
                Some(inner_response) => {
                    let response = lua.create_userdata(Response::from(inner_response.response))?;
                    match inner_response.inner {
                        Some(mut mv) => {
                            mv.push_front(Value::UserData(response));
                            mv
                        }
                        None => MultiValue::from_iter([Value::UserData(response)].into_iter()),
                    }
                }
                None => MultiValue::new(),
            };
            multi_value.push_front(unsafe { tealr::mlu::mlua::Value::Boolean(WINDOW_OPEN) });
            Ok(multi_value)
        });
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(_fields: &mut F) {}
}

wrapper!(Resize egui::containers::Resize);
impl TealData for Resize {}
wrapper!(Frame egui::Frame);

impl TealData for Frame {}
