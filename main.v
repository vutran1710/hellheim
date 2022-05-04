import vweb

struct App {
	vweb.Context
}

fn main() {
	mut app := App{}
	vweb.run(app, 8081)
}
