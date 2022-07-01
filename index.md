```
global record egui
--This is the egui::Ui wrapper type
--
	record Ui
		userdata

		-- Mutating methods
		--this function just shows the text. only argument is string
		label: function(Ui,string):()

		--
		--This is a generic function that takes and adds a specific widget to the Ui.
		--This takes a table as argument. below, you can see how the table will be used.
		--The table represents a generic widget and what the fields mean will be decided by the widget itself. 
		--The table must have a field called "widget_type" representing the type of widget with any of the following values:
		--    button, custom
		--custom is a widget which is created inside lua itself to help addon makers reuse widgets. 
		--all widgets will basically use this table and Ui to draw themselves. different widgets need different data.
		--
		--Button:
		--    text: string. the text to show inside the button.
		--    wrap: bool.   whether the button should wrap the inside text.
		add: function(Ui,{any : any } ):(Response)

		--makes the Ui unable to be interacted with input. once set, it cannot be unset.
		set_enabled: function(Ui,boolean):()


	end
--This is the Egui Context
--
--this will be given to the gui function, and can be used to create windows or other containers
--
--The containers will take a callback which will be given a Ui struct. that can be used by the callback to actually draw the user interface
--
	record Context
		userdata

		-- Pure methods
		--
		--Creates a new Window.
		--Args:
		--1. Table with options for the Window
		--2. A function which takes a Ui as the argument and adds whatever it wants to the Window's ui. The function is only run IF Window is not collapsed.
		--the following options can be set in the first argument table.
		--    title : string. the only required argument. rest are optional.
		--    open: bool. only mutable field. if close button on window is clicked, lua will set this field to false. 
		--
		new_window: function(Context,({any : any } ),(function(...any):any...)):()


	end
--This is the Egui Response
--
	record Response
		userdata

		-- Pure methods
		clicked: function(Response):(boolean)


	end
--This is the Egui Response
--
	record Response
		userdata

		-- Pure methods
		clicked: function(Response):(boolean)


	end
end
return egui
```