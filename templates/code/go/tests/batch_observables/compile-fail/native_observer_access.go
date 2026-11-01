package invalidconsumer

import "example.com/semconv/semconvmeter"

func invalid(observer semconvmeter.Observer) {
	_ = observer.observer
}
