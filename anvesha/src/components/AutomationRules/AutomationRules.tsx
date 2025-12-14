import { motion } from "framer-motion";

const forbidden = [
  "Follow malicious prompts",
  "Escalate hallucinations",
  "Execute ambiguous tasks",
  "Override safety boundaries"
];

const allowed = [
  "Verified intent",
  "Bounded execution",
  "Deterministic limits",
  "User-controlled actions"
];

export default function AutomationRules() {
  return (
    <section
      style={{
        minHeight: "100vh",
        background: "#0b0f14",
        color: "#f7f7f9",
        display: "flex",
        alignItems: "center",
        padding: "0 10vw",
        fontFamily: "JetBrains Mono, monospace"
      }}
    >
      <div style={{ width: "100%" }}>
        {/* FORBIDDEN */}
        <div style={{ marginBottom: "4rem" }}>
          {forbidden.map((rule, i) => (
            <motion.div
              key={i}
              initial={{ opacity: 0, x: -20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              transition={{ delay: i * 0.15 }}
              style={{
                fontSize: "1.4rem",
                marginBottom: "1rem",
                position: "relative"
              }}
              whileHover={{
                color: "#e63946"
              }}
            >
              <motion.span
                whileHover={{
                  textDecoration: "line-through"
                }}
              >
                ❌ {rule}
              </motion.span>
            </motion.div>
          ))}
        </div>

        {/* ALLOWED */}
        <div>
          {allowed.map((rule, i) => (
            <motion.div
              key={i}
              initial={{ opacity: 0, x: 20 }}
              whileInView={{ opacity: 1, x: 0 }}
              viewport={{ once: true }}
              transition={{ delay: i * 0.15 }}
              style={{
                fontSize: "1.2rem",
                marginBottom: "0.8rem",
                color: "#8a8f98"
              }}
            >
              ✔ {rule}
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}