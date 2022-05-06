module interfaces

import net.http

pub struct Module<T, K> {
	state   T
	handler fn (http.Request) ?K
}
