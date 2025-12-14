import { threads } from "./timeline.data";
import "./timelineThread.css";

export default function TimelineThread() {
  return (
    <section className="timeline-thread">
      {/* THREAD LINES */}
      <svg
  className="timeline-svg"
  viewBox="0 0 100 100"
  preserveAspectRatio="none"
>
  {threads.map((thread) => (
    <polyline
      key={thread.id}
      points={thread.points
        .map((p, idx) => {
          const drift = idx % 2 === 0 ? -6 : 6; // crossing
          return `${thread.lane + drift},${p.y}`;
        })
        .join(" ")}
      fill="none"
      stroke={thread.visible ? "#8a8f98" : "#555"}
      strokeWidth="0.4"
      strokeDasharray={thread.visible ? "0" : "2 2"}
      opacity={thread.visible ? 0.6 : 0.35}
    />
  ))}
</svg>

      {/* NODES */}
      <div className="timeline-nodes">
        {threads.map((thread, i) =>
          thread.points.map((p, idx) => (
            <div
  key={`${thread.id}-${idx}`}
  className={`node ${thread.visible ? "" : "redacted"}`}
  style={{
    top: `${p.y}%`,
    left: `${thread.lane}%`
  }}
>
  {p.text}
</div>
          ))
        )}
      </div>
    </section>
  );
}