module servers

import net.http { CommonHeader, Request, Response, Server }

struct Handler {}

fn (h Handler) handle(req Request) Response {
	result := h.next(req)
	mut res := Response{
		header: http.new_header_from_map({
			CommonHeader.content_type: 'text/plain'
		})
	}
	res.text = result
	return res
}

pub fn start(ops fn (r Request)) ? {
	handler := Handler{}
	handler.next = ops
	mut server := Server{
		handler: handler
	}
	server.listen_and_serve() ?
}
