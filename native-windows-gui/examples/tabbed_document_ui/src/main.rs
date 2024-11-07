extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use nwg::{ControlHandle, Event, Frame, NativeUi, NwgError, StatusBar, Tab, TabsContainer};

use nwg::stretch::{geometry::{Size, Rect}, style::{Dimension, FlexDirection, AlignSelf}};

const PT_10: Dimension = Dimension::Points(10.0);
const PT_5: Dimension = Dimension::Points(5.0);

const PADDING: Rect<Dimension> = Rect{ start: PT_10, end: PT_10, top: PT_10, bottom: PT_10 };

#[derive(Default)]
pub struct App {
    window: nwg::Window,

    layout: nwg::FlexboxLayout,

    toolbar: ToolBar,

    tabs_container: TabsContainer,

    tabs: Vec<Tab>,

    status_bar: StatusBar
}

#[derive(Default)]
pub struct ToolBar {
    frame: nwg::Frame,
    home_button: nwg::Button,
    new_button: nwg::Button,
    open_button: nwg::Button,
    close_all_button: nwg::Button,
}

impl ToolBar {
    pub fn build<P: Into<ControlHandle>>(&mut self, parent: P) -> Result<(), NwgError> {
        nwg::Frame::builder()
            .flags(nwg::FrameFlags::VISIBLE)
            .parent(parent)
            .build(&mut self.frame)?;

        // FIXME find a way to make a row of buttons with automatic positioning and size
        //       if width/height are not specified all the buttons appear on top of each other
        //       with a default size.

        nwg::Button::builder()
            .text("Home")
            .size((100, 32))
            .position((0, 0))
            .parent(&self.frame)
            .focus(true)
            .build(&mut self.home_button)?;

        nwg::Button::builder()
            .text("New")
            .size((100, 32))
            .position((100, 0))
            .parent(&self.frame)
            .focus(true)
            .enabled(false)
            .build(&mut self.new_button)?;

        nwg::Button::builder()
            .text("Open")
            .size((100, 32))
            .position((200, 0))
            .parent(&self.frame)
            .focus(true)
            .enabled(false)
            .build(&mut self.open_button)?;

        nwg::Button::builder()
            .text("Close all")
            .size((100, 32))
            .position((300, 0))
            .parent(&self.frame)
            .focus(true)
            .build(&mut self.close_all_button)?;

        Ok(())
    }
}

pub struct AppUi {
    inner: App,
    default_handler: RefCell<Vec<nwg::EventHandler>>
}

impl Deref for AppUi {
    type Target = App;

    fn deref(&self) -> &App {
        &self.inner
    }
}

impl AppUi {

    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn destroy(&self) {
        println!("BasicAppUi::destroy");
        let mut handlers = self.default_handler.borrow_mut();
        for handler in handlers.drain(0..) {
            nwg::unbind_event_handler(&handler);
        }
    }
}

impl nwg::NativeUi<Rc<AppUi>> for App {
    fn build_ui(mut inital_state: Self) -> Result<Rc<AppUi>, NwgError> {

        // Window
        nwg::Window::builder()
            .size((800, 600))
            .position((300, 300))
            .title("Tabbed Document UI")
            .build(&mut inital_state.window)?;

        inital_state.toolbar.build(&inital_state.window)?;

        TabsContainer::builder()
            .parent(&inital_state.window)
            .build(&mut inital_state.tabs_container)?;

        let mut home_tab: Tab = Default::default();
        Tab::builder()
            .text("Home")
            .parent(&inital_state.tabs_container)
            .build(&mut home_tab)?;

        inital_state.tabs.push(home_tab);


        nwg::StatusBar::builder()
            .parent(&inital_state.window)
            .text("Status")
            .build(&mut inital_state.status_bar)?;

        nwg::FlexboxLayout::builder()
            .parent(&inital_state.window)
            .flex_direction(FlexDirection::Column)
            .child(&inital_state.toolbar.frame)
                .child_size(Size { width: Dimension::Percent(100.0), height: Dimension::Points(32.0) })
            .child(&inital_state.tabs_container)
                .child_size(Size { width: Dimension::Percent(100.0), height: Dimension::Points(32.0) })
            .build(&inital_state.layout)?;

        let ui = Rc::new(AppUi {
            inner: inital_state,
            default_handler: Default::default()
        });

        // Events
        let mut window_handles = vec![&ui.window.handle];
        for handle in window_handles.iter() {
            let evt_ui = ui.clone();
            let handle_events = move |evt, evt_data, handle| {
                match evt {
                    Event::OnWindowClose => {
                        println!("Event::OnWindowClose");

                        if &handle == &evt_ui.window {
                            App::exit(&evt_ui);
                        }
                    }
                    _ => {}
                }
            };

            ui.default_handler.borrow_mut().push(
                nwg::full_bind_event_handler(handle, handle_events)
            );
        }


        return Ok(ui);
    }
}

impl App {
    fn exit(&self) {
        println!("BasicApp::exit");
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let ui: Rc<AppUi> = App::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();

    ui.destroy();

    println!("done");
}
