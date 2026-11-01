package invalidconsumer

import "example.com/semconv/semconvmeter"

func invalid(handle semconvmeter.Observable) {
	_ = handle.Instrument()
}
