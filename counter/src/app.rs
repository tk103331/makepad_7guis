use makepad_widgets::*;

// The live_design macro generates a function that registers a DSL code block with the global
// context object (`Cx`).
//
// DSL code blocks are used in Makepad to facilitate live design. A DSL code block defines
// structured data that describes the styling of the UI. The Makepad runtime automatically
// initializes widgets from their corresponding DSL objects. Moreover, external programs (such
// as a code editor) can notify the Makepad runtime that a DSL code block has been changed, allowing
// the runtime to automatically update the affected widgets.
live_design! {
    import makepad_widgets::button::Button;
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::view::View;
    import makepad_widgets::label::Label;
    
    // The `{{App}}` syntax is used to inherit a DSL object from a Rust struct. This tells the
    // Makepad runtime that our DSL object corresponds to a Rust struct named `App`. Whenever an
    // instance of `App` is initialized, the Makepad runtime will obtain its initial values from
    // this DSL object.
    App = {{App}} {
        // The `ui` field on the struct `App` defines a view widget. Views are used as containers
        // for other widgets. Since the `ui` property on the DSL object `App` corresponds with the
        // `ui` field on the Rust struct `App`, the latter will be initialized from the DSL object
        // here below.
        ui: <DesktopWindow>{
            
            show_bg: true

            // determines how the view widget itself is laid out. In this
            // case, the view widget takes up the entire window.
            width: Fill,
            height: Fill
            
            draw_bg: {
                
                // The `fn pixel(self) -> vec4` syntax is used to define a property named `pixel`,
                // the value of which is a shader. We use our own custom DSL to define shaders. It's
                // syntax is *mostly* compatible with GLSL, although there are some differences as
                // well.
                fn pixel(self) -> vec4 {
                    // Within a shader, the `self.geom_pos` syntax is used to access the `geom_pos`
                    // attribute of the shader. In this case, the `geom_pos` attribute is built in,
                    // and ranges from 0 to 1.
                    return mix(#7, #3, self.pos.y);
                }
            }
            
            // The `name:` syntax is used to define fields, i.e. properties for which there are
            // corresponding struct fields. In contrast, the `name =` syntax is used to define
            // instance properties, i.e. properties for which there are no corresponding struct
            // fields. Note that fields and instance properties use different namespaces, so you
            // can have both a field and an instance property with the same name.
            //
            // Widgets can hook into the Makepad runtime with custom code and determine for
            // themselves how they want to handle instance properties. In the case of view widgets,
            // they simply iterate over their instance properties, and use them to instantiate their
            // child widgets.
            <View> {
                // The properties below determines how child widgets are laid out within a view. In
                // this case, child widgets flow downward, with 20 pixels of spacing in between them,
                // and centered horizontally with respect to the entire view.
                //
                // Because the child widgets flow downward, vertical alignment works somewhat
                // differently. In this case, children are centered vertically with respect to the
                // remainder of the view after the previous children have been drawn.
                flow: Right,
                spacing: 20,
                align: {
                    x: 0.5,
                    y: 0.5
                }
                // A label to display the counter.
                label = <Label> {
                    draw_text: {
                        color: #f
                    },
                    text: "0"
                }

                // A button to increment the counter.
                //
                // The `<Button>` syntax is used to inherit a DSL object from another DSL object. This
                // tells the Makepad runtime our DSL object has the same properties as the DSL object
                // named `Button`, except for the properties defined here below, which override any
                // existing values.
                
                button = <Button> {
                    text: "Count"
                }
            }
        }
    }
}

// This app_main macro generates the code necessary to initialize and run your application.
//
// This code is almost always the same between different applications, so it is convenient to use a
// macro for it. The two main tasks that this code needs to carry out are: initializing both the
// main application struct (`App`) and the global context object (`Cx`), and setting up event
// handling. On desktop, this means creating and running our own event loop. On web, this means
// creating an event handler function that the browser event loop can call into.
app_main!(App);

// The main application struct.
//
// The #[derive(Live, LiveHook)] attribute implements a bunch of traits for this struct that enable
// it to interact with the Makepad runtime. Among other things, this enables the Makepad runtime to
// initialize the struct from a DSL object.
#[derive(Live)]
// This function is used to register any DSL code that you depend on.
// called automatically by the code we generated with the call to the macro `main_app` above.

pub struct App {
    // A chromeless window for our application. Used to contain our view widget.
    // A view widget. Used to contain our button and label.
    #[live] ui: WidgetRef,
    
    // The value for our counter.
    //
    // The #[rust] attribute here is used to indicate that this field should *not* be initialized
    // from a DSL object, even when a corresponding property exists.
    #[rust] counter: usize,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl App {
    async fn _do_network_request(_cx:CxRef, _ui:WidgetRef, _url:&str)->String{
        //let x = fetch(urL).await;
        //ui.get_label(id!(thing)).set_text(&mut *cx.borrow_mut(), x);
        "".to_string()
    }
}

impl AppMain for App{
    
    // This function is used to handle any incoming events from the host system. It is called
    // automatically by the code we generated with the call to the macro `main_app` above.
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            // This is a draw event, so create a draw context and use that to draw our application.
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }
        
        // Forward the event to the view. In this case, handle_event returns a list of actions.
        // Actions are similar to events, except that events are always forwarded downward to child
        // widgets, while actions are always returned back upwards to parent widgets.
        let actions = self.ui.handle_widget_event(cx, event);
        
        // Get a reference to our button from the view, and check if one of the actions returned by
        // the view was a notification that the button was clicked.
        if self.ui.button(id!(button)).clicked(&actions) {
            //cx.spawn_async(Self::do_network_request(cx.get_ref(), self.ui.clone()))
            // Increment the counter.
            self.counter += 1;
            
            // Get a reference to our label from the view, update its text, and schedule a redraw
            // for it.
            let label = self.ui.label(id!(label));
            label.set_text(&format!("{}", self.counter));
            label.redraw(cx);
        }
    }
}