import { motion } from "framer-motion";

const lines = [
  "ANVESHA IS A PRIVATE SEARCH BROWSER.",
  "",
  "The browser is not a shell.",
  "It is the execution boundary.",
  "",
  "Search is not outsourced.",
  "Indexing, inference, and summarization run locally.",
  "",
  "The core system is written in Rust.",
  "Memory safety is not optional.",
  "Determinism is not negotiable."
];

export default function SystemArchitecture() {
  return (
    <section
      style={{
        minHeight: "80vh",
        display: "flex",
        alignItems: "center",
        paddingLeft: "10vw",
        fontFamily: "JetBrains Mono, monospace",
        color: "#f7f7f9",
        position: "relative",
        zIndex: 3,
        background: "transparent" // 👈 critical for grain
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
        style={{ maxWidth: "680px" }}
      >
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
                fontSize: "1.05rem",
                lineHeight: 1.8,
                letterSpacing: "0.04em"
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