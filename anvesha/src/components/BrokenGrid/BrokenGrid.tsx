import { motion, useScroll, useTransform } from "framer-motion";
import { useRef } from "react";
import "./brokenGrid.css";

export default function BrokenGrid() {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start end", "end start"]
  });

  const ySlow = useTransform(scrollYProgress, [0, 1], [40, -40]);
  const yFast = useTransform(scrollYProgress, [0, 1], [80, -80]);

  return (
    <section ref={ref} className="broken-grid">
      <motion.div style={{ y: ySlow }} className="bg-item left">
        LOCAL INTELLIGENCE
      </motion.div>

      <motion.div style={{ y: yFast }} className="bg-item right">
        ZERO SILENT LEAKS
      </motion.div>

      <motion.div style={{ y: ySlow }} className="bg-item center rotated">
        USER OWNED CONTEXT
      </motion.div>
    </section>
  );
}