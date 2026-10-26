# Generate and use metrics

From a checkout of this repository, generate a module into your application:

```sh
mise exec -- weaver registry generate \
  --v2 \
  -r path/to/registry \
  -t templates \
  code/typescript_effect \
  path/to/app/src/semconv
```

For a released package, replace `-t templates` with
`-t 'https://github.com/TrogonStack/trogonstack-weaver-packages.git@v{version}[templates]'`.
The consuming application needs `effect`; the verified version is 4.0.5.

Define metrics in your application's registry. For example:

```yaml
file_format: definition/2
attributes:
  - key: worker.result
    type:
      members:
        - id: success
          value: success
          brief: The task succeeded.
          stability: stable
        - id: failure
          value: failure
          brief: The task failed.
          stability: stable
    brief: The task result.
    stability: stable
metrics:
  - name: worker.task.duration
    brief: Time spent running a task.
    instrument: histogram
    unit: s
    stability: stable
    requirement_level: recommended
    attributes:
      - ref: worker.result
        requirement_level: required
```

The registry also needs a `manifest.yaml` with its name and schema URL, such as:

```yaml
name: worker
schema_url: https://example.com/schemas/worker/1.0.0
```

Call the generated helper inside your Effect program:

```ts
import * as Duration from "effect/Duration";
import * as Effect from "effect/Effect";
import { updateWorkerTaskDuration, WorkerResult } from "./semconv/metrics.ts";

const program = Effect.gen(function* () {
  yield* updateWorkerTaskDuration(Duration.millis(250), {
    result: WorkerResult.Success,
  });
});
```

This records `0.25` in the seconds histogram. Run the program with your
application's existing Effect runtime and OTLP metrics layer.

Keep application code outside the generated directory. Weaver does not remove
stale generated files; clear that directory before regenerating after changing
the output layout.
