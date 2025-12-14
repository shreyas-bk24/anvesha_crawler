import { nodes, edges } from "./data";
import "./wiregraph.css";

export default function WireGraphTimeline() {
  const nodeMap = Object.fromEntries(
    nodes.map(n => [n.id, n])
  );

  return (
    <section className="wiregraph">
      <div
  style={{
    position: "absolute",
    top: "6vh",
    left: "10vw",
    fontFamily: "JetBrains Mono, monospace",
    fontSize: "0.9rem",
    letterSpacing: "0.18em",
    opacity: 0.6,
    zIndex: 4
  }}
>
  SYSTEM EVOLUTION MAP
</div>
      {/* WIRES */}
      <svg
        className="wiregraph-svg"
        viewBox="0 0 100 100"
        preserveAspectRatio="none"
      >
        {edges.map((edge, i) => {
          const a = nodeMap[edge.from];
          const b = nodeMap[edge.to];

          // control point (same as before)
          const ctrlX = (a.x + b.x) / 2;
          const ctrlY =
            (a.y + b.y) / 2 - (edge.type === "branch" ? 6 : 10);

          // tangent at endpoint = end - control point
          const tx = b.x - ctrlX;
          const ty = b.y - ctrlY;
          const tLen = Math.sqrt(tx * tx + ty * ty) || 1;

          // pull back along tangent
          const pull = 1;
          const endX = b.x - (tx / tLen) * pull;
          const endY = b.y - (ty / tLen) * pull;

          return (
            <path
              key={i}
              d={`M ${a.x} ${a.y} Q ${ctrlX} ${ctrlY} ${endX} ${endY}`}
              fill="none"
              stroke="#8a8f98"
              strokeWidth={edge.type === "branch" ? "0.25" : "0.45"}
              opacity={edge.type === "branch" ? 0.3 : 0.5}
              strokeDasharray={edge.type === "branch" ? "2 3" : "0"}
            />
          );
        })}
      </svg>
    
    {/* NODES */}
{nodes.map(n => {
  // find incoming edge (wire ending at this node)
  const incoming = edges.find(e => e.to === n.id);

  let offsetX = 10;
  let offsetY = 0;

  if (incoming) {
    const from = nodeMap[incoming.from];
    const dx = n.x - from.x;
    const dy = n.y - from.y;

    const len = Math.sqrt(dx * dx + dy * dy) || 1;

    // offset label in direction of wire
    offsetX = (dx / len) * 12;
    offsetY = (dy / len) * 12;
  }

  return (
   <div
  key={n.id}
  className={`wg-node ${n.visible ? "" : "redacted"}`}
  style={{
    left: `${n.x}%`,
    top: `${n.y}%`
  }}
>
  <span className="dot" />

  <span
    className="label"
    style={{
      transform: `translate(${offsetX}px, ${offsetY}px)`
    }}
  >
    {n.label}
  </span>
</div>
  );
})}
      <div
  style={{
    position: "absolute",
    bottom: "8vh",
    right: "8vw",
    fontFamily: "JetBrains Mono, monospace",
    fontSize: "0.65rem",
    letterSpacing: "0.14em",
    lineHeight: 1.8,
    opacity: 0.45,
    zIndex: 4,
    textAlign: "right"
  }}
>
  ● ACTIVE NODE<br />
  ⋯⋯ BRANCH PATH<br />
  — MAIN PATH<br />
  ██ REDACTED
</div>
    </section>
  );
}