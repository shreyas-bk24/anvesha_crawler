import { motion } from "framer-motion";
import WireBackground from "./WireBackground";

export default function Manifesto() {
  return (
    <section
      style={{
        position: "relative",
        minHeight: "100vh",
        background: "#0b0f14",
        overflow: "hidden"
      }}
    >
      {/* Background wires */}
      <WireBackground />

      {/* CONTENT WRAPPER — THIS FIXES THE ISSUE */}
      <div
        style={{
          position: "relative",
          zIndex: 2,
          minHeight: "100vh",
          display: "flex",
          alignItems: "center",
          paddingLeft: "10vw",
          fontFamily: "JetBrains Mono, monospace",
          color: "#f7f7f9"
        }}
      >
        <div>
          {/* BLOCK 1 */}
          <motion.p
            initial={{ opacity: 0, y: 12 }}
            whileInView={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.3, ease: "linear" }}
            viewport={{ once: true }}
            style={{
              fontSize: "1.3rem",
              letterSpacing: "0.04em",
              marginBottom: "1.2rem"
            }}
          >
            Search was built to observe you.
          </motion.p>

          {/* BLOCK 2 */}
          <motion.div
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            transition={{ duration: 0.3, ease: "linear" }}
            viewport={{ once: true }}
            style={{ marginBottom: "1.8rem" }}
          >
            <p>We built something that refuses.</p>
            <p>Anvesha does not send queries outward.</p>
            <p>It pulls intelligence inward.</p>
          </motion.div>

          {/* BLOCK 3 */}
          <motion.div
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            transition={{ duration: 0.3, ease: "linear" }}
            viewport={{ once: true }}
            style={{
              marginBottom: "2.4rem",
              color: "#8a8f98"
            }}
          >
            <p>No cloud-first.</p>
            <p>No blind trust.</p>
            <p>No hallucinated authority.</p>
          </motion.div>

          {/* ASSERTION */}
          <motion.div
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            transition={{ duration: 0.35, ease: "linear" }}
            viewport={{ once: true }}
            style={{
              borderLeft: "2px solid #e63946",
              paddingLeft: "1.5rem"
            }}
          >
            <p style={{ fontSize: "2rem", letterSpacing: "0.08em" }}>
              THIS IS NOT A BROWSER.
            </p>
            <p style={{ fontSize: "2rem", letterSpacing: "0.08em" }}>
              THIS IS A CONTAINMENT LAYER.
            </p>
          </motion.div>
        </div>
      </div>
      {/* Ambient system metadata */}
<div
  style={{
    position: "absolute",
    bottom: "6vh",
    left: "10vw",
    fontFamily: "JetBrains Mono, monospace",
    fontSize: "0.75rem",
    letterSpacing: "0.12em",
    opacity: 0.4,
    color: "#8a8f98"
  }}
>
  <div>SYS-ID: ANV-MNF-001</div>
  <div>STATE: VERIFIED</div>
</div>
    </section>
  );
}