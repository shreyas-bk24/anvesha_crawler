import { motion, useScroll, useTransform } from "framer-motion";
import { useRef } from "react";
import {
  disclosedTimeline,
  redactedTimeline
} from "./timeline.data";
import "./timeline.css";

export default function TimelineSplit() {
  const ref = useRef<HTMLDivElement>(null);

  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start end", "end start"]
  });

  // Resistance effect
  const rightY = useTransform(scrollYProgress, [0, 1], ["0%", "-30%"]);

  return (
    <section ref={ref} className="timeline-root">
      {/* LEFT — DISCLOSED */}
      <div className="timeline-left">
        <h3>DISCLOSED</h3>
        {disclosedTimeline.map((item, i) => (
          <div key={i} className="timeline-item">
            <span className="date">{item.date}</span>
            <span className="title">{item.title}</span>
          </div>
        ))}
      </div>

      {/* RIGHT — REDACTED */}
      <motion.div className="timeline-right" style={{ y: rightY }}>
        <h3>REDACTED</h3>
        {redactedTimeline.map((item, i) => (
          <div key={i} className="timeline-item redacted">
            <span className="date">{item.date}</span>
            <span className="title">{item.title}</span>
          </div>
        ))}
      </motion.div>
      
    </section>
  );
}