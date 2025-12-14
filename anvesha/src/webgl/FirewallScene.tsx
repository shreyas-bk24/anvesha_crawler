import { useFrame } from "@react-three/fiber";
import { useRef } from "react";
import * as THREE from "three";
import { MotionValue } from "framer-motion";

import vert from "./shaders/firewall.vert.glsl";
import frag from "./shaders/firewall.frag.glsl";

export default function FirewallScene({
  intensity
}: {
  intensity: MotionValue<number>;
}) {
  const material = useRef<THREE.ShaderMaterial>(null!);

  useFrame(({ clock }) => {
    if (!material.current) return;

    material.current.uniforms.uTime.value =
      clock.getElapsedTime();

    material.current.uniforms.uIntensity.value =
      intensity.get();
  });

  return (
    <mesh>
      <planeGeometry args={[6, 6, 1, 1]} />
      <shaderMaterial
        ref={material}
        vertexShader={vert}
        fragmentShader={frag}
        transparent
        blending={THREE.AdditiveBlending}
        depthWrite={false}
        uniforms={{
          uTime: { value: 0 },
          uIntensity: { value: 0.5 }
        }}
      />
    </mesh>
  );
}