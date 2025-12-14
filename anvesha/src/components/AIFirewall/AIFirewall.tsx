import { useMotionValue } from "framer-motion";
import CanvasRoot from "../../webgl/CanvasRoot";
import FirewallScene from "../../webgl/FirewallScene";
import { useScrollIntensity } from "../../hooks/useScrollIntensity";

export default function AIFirewall() {
  const intensity = useScrollIntensity(0.4, 1.4);

  return (
    <section
      style={{
        position: "relative",
        minHeight: "120vh",
        background: "#0b0f14"
      }}
    >
      {/* Firewall metadata */}
<div
  style={{
    position: "absolute",
    top: "6vh",
    right: "6vw",
    fontFamily: "JetBrains Mono, monospace",
    fontSize: "0.75rem",
    letterSpacing: "0.12em",
    opacity: 0.45,
    color: "#8a8f98",
    textAlign: "right",
    zIndex: 12
  }}
>
  <div>FW-STATUS: ACTIVE</div>
  <div>THREAT VECTOR: MONITORED</div>
</div>
      {/* WebGL Layer */}
      <div
        style={{
          position: "absolute",
          inset: 0,
          zIndex: 1
        }}
      >
        <CanvasRoot>
          <FirewallScene intensity={intensity} />
        </CanvasRoot>
      </div>

      {/* UI Overlay */}
      <div
        style={{
          position: "relative",
          zIndex: 10,
          height: "100%",
          display: "flex",
          alignItems: "center",
          paddingLeft: "10vw",
          color: "#f7f7f9",
          fontFamily: "JetBrains Mono, monospace",
        }}
      >
        <pre style={{ lineHeight: 1.6 }}>

FIREWALL STATUS: ACTIVE

INPUT STREAM: DETECTED
INTENT VECTOR: ANALYZED
MANIPULATION RISK: 0.92

ACTION: BLOCKED

        </pre>
      </div>
    </section>
  );
}