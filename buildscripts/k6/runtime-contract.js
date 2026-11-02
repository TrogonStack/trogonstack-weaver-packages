import { Counter } from "k6/metrics";

for (const name of ["x", "invalid.dot", "n".repeat(130)]) {
  let rejected = false;
  try { new Counter(name); }
  catch (error) { rejected = error.message.includes("Invalid metric name"); }
  if (!rejected) throw new Error("Pinned native name restriction changed");
}
const shortest = new Counter("nn");
const longest = new Counter("n".repeat(129));
export default function () {
  shortest.add(1);
  longest.add(1);
}
