use cursive::views::Dialog;
use cursive::Cursive;

fn main() {
    // Creates the cursive root - required for every application.
    let mut siv = cursive::default();

    // Creates a dialog with a single "Quit" button
    siv.add_layer(
        Dialog::text("Select :")
            .title("RustyMinitel")
            .button("Informations", information)
            .button("Network", network)
            .button("Process", process)
            .button("Quit", |_q| _q.quit()),
    );

    // Starts the event loop.
    siv.run();
}

fn information(s: &mut Cursive) {
    /*_s.add_layer(Dialog::info("Try again!")) → Debug*/
    s.pop_layer();
    s.add_layer(
        Dialog::text("Informations User")
            .title("RustyMinitel / Informations")
            .button("Exit", menu),
    );
}

fn network(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Network User")
            .title("RustyMinitel / Network")
            .button("Return Menu", menu),
    );
}

fn process(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Process")
            .title("RustyMinitel / Process")
            .button("Return Menu", menu),
    );
}

fn menu(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Select :")
            .title("RustyMinitel")
            .button("Informations", information)
            .button("Network", network)
            .button("Process", process)
            .button("Quit", |_q| _q.quit()),
    );
}
