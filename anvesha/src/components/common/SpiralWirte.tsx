export default function SpiralWire({
  size = 600,
  stroke = "#8a8f98",
  opacity = 0.06,
  duration = 35
}: {
  size?: number;
  stroke?: string;
  opacity?: number;
  duration?: number;
}) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 600 600"
      style={{
        position: "absolute",
        top: "50%",
        left: "50%",
        transform: "translate(-50%, -50%)",
        opacity,
        pointerEvents: "none",
        animation: `spiralRotate ${duration}s linear infinite`
      }}
    >
      <path
        d="
          M300 300
          m -5 0
          a 5 5 0 1 1 10 0
          a 10 10 0 1 1 -20 0
          a 20 20 0 1 1 40 0
          a 40 40 0 1 1 -80 0
          a 80 80 0 1 1 160 0
          a 120 120 0 1 1 -240 0
          a 180 180 0 1 1 360 0
        "
        fill="none"
        stroke={stroke}
        strokeWidth="1"
        strokeLinecap="round"
      />
    </svg>
  );
}