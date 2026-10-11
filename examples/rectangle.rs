use rust_ui::{
    Color,
    Column,
    Row,
    Rectangle,
    Size,
    WindowApp,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Column::new()
        .spacing(10.0)
        .child(
            Rectangle::new(
                Size::new(200.0, 100.0),
                Color::rgb(255, 0, 0),
            )
        )
        .child(
            Rectangle::new(
                Size::new(500.0, 80.0),
                Color::rgb(0, 255, 0),
            )
        );


    let root = Row::new()
        .spacing(10.0)
        .child(root)
        .child(
            Rectangle::new(
                Size::new(5000.0, 80.0),
                Color::rgb(0, 255, 0),
            )
        );



    WindowApp::new(root).run()


    
}