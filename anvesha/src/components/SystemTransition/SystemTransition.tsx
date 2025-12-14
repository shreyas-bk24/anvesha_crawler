import { motion } from "framer-motion";
import SpiralWire from "../common/SpiralWirte";

const lines = [
  "— SYSTEM STATE —",
  "",
  "Context locked",
  "External trust disabled",
  "Inference boundaries enforced",
  "",
  "Execution surface: PRIVATE BROWSER",
  "Search layer: LOCAL",
  "Core runtime: RUST"
];

export default function SystemTransition() {
  return (
    <section
      style={{
        minHeight: "60vh",
        display: "flex",
        alignItems: "center",
        paddingLeft: "10vw",
        fontFamily: "JetBrains Mono, monospace",
        color: "#8a8f98",
        position: "relative",
        zIndex: 3
      }}
    >
      <motion.div
        initial="hidden"
        whileInView="visible"
        viewport={{ once: true }}
        variants={{
          visible: {
            transition: {
              staggerChildren: 0.2
            }
          }
        }}
      >
        <SpiralWire
  size={520}
  opacity={0.05}
  duration={40}
/>
        {lines.map((line, i) =>
          line === "" ? (
            <div key={i} style={{ height: "1rem" }} />
          ) : (
            <motion.div
              key={i}
              variants={{
                hidden: { opacity: 0, y: 6 },
                visible: { opacity: 1, y: 0 }
              }}
              transition={{
                duration: 0.22,
                ease: "linear"
              }}
              style={{
                fontSize: "0.95rem",
                letterSpacing: "0.14em",
                lineHeight: 1.8
              }}
            >
              {line}
            </motion.div>
          )
        )}
      </motion.div>
    </section>
  );
}