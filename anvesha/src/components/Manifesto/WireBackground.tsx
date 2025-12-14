export default function WireBackground() {
  return (
    <svg
      viewBox="0 0 1000 600"
      preserveAspectRatio="none"
      style={{
        position: "absolute",
        inset: 0,
        width: "100%",
        height: "100%",
        opacity: 0.06,
        pointerEvents: "none",
        animation: "wireDrift 20s linear infinite",
      }}
    >
      <path
        d="M50 100 L300 120 L450 80 L700 140 L900 110"
        stroke="#8a8f98"
        strokeWidth="1"
        fill="none"
      />
      <path
        d="M120 300 L260 280 L420 320 L680 290 L850 340"
        stroke="#8a8f98"
        strokeWidth="0.8"
        fill="none"
      />
      <path
        d="M80 480 L240 460 L390 500 L620 470 L880 520"
        stroke="#e63946"
        strokeWidth="0.6"
        fill="none"
      />
    </svg>
  );
}