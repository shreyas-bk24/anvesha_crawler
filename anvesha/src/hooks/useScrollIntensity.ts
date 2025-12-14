import { useScroll, useSpring, useTransform } from "framer-motion";

export function useScrollIntensity(
  min = 0.3,
  max = 1.2
) {
  const { scrollYProgress } = useScroll();

  // Map scroll progress to intensity range
  const raw = useTransform(scrollYProgress, [0, 1], [min, max]);

  // Smooth it (mechanical, not bouncy)
  const intensity = useSpring(raw, {
    stiffness: 120,
    damping: 30,
    mass: 0.6
  });

  return intensity;
}