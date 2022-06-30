```
global record egui
--This is the Egui Context
--
--this will be given to the gui function, and can be used to create windows or other containers
--
--The containers will take a callback which will be given a Ui struct. that can be used by the callback to actually draw the user interface
--
	record Context
		userdata

		-- Pure methods
		--takes a table which has the relevant options set and a ui callback which will be called if window is not collapsed
		--
		--the following fields maybe set in the table. only title is required and rest are optional
		--
		--title : string
		--
		--open: bool. true is window shown or open. false is window not displayed or closed. we will set this field if user clicks the close button on top right of window
		new_window: function(Context,({any : any } ),(function(...any):any...)):()


	end
--This is the egui::Ui wrapper type
--
	record Ui
		userdata

		-- Mutating methods
		--this function just shows a text as a label
		label: function(Ui,string):()


	end
end
return egui
```