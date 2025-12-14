import { Canvas } from "@react-three/fiber";

export default function CanvasRoot({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <Canvas
      dpr={[1, 1.5]}
      camera={{ position: [0, 0, 6], fov: 45 }}
      gl={{ antialias: false, alpha: true }}
    >
      {children}
    </Canvas>
  );
}