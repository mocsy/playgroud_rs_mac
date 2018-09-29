extern crate azul;

use azul::prelude::*;
use  azul::widgets::{ button::Button, label::Label,
 table_view::TableView, text_input::TextInput  };

struct CounterApplication {
    counter: usize,
}

impl Layout for CounterApplication {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("Update counter").dom()
            .with_callback(On::MouseUp, Callback(update_counter));

        //let table = TableView::new().dom();
        //let input = TextInput::new(format!("{}", self.counter)).dom();

        Dom::new(NodeType::Div)
            .with_child(label)
            .with_child(button)
            //.with_child(table)
            //.with_child(input)
    }
}

fn update_counter(app_state: &mut AppState<CounterApplication>, _event: WindowEvent) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    UpdateScreen::Redraw
}

fn main() {
    let app = App::new(CounterApplication { counter: 0 }, AppConfig::default());
    app.run(Window::new(WindowCreateOptions::default(), Css::native()).unwrap()).unwrap();
}
