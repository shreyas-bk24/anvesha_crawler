import { useEffect, useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { lockScroll, unlockScroll } from "../../hooks/useScrollLock";
import "./entryGate.css";

export default function EntryGate() {
  const [visible, setVisible] = useState(true);

  useEffect(() => {
    lockScroll();

    const timer = setTimeout(() => {
      setVisible(false);
      unlockScroll();
    }, 2000); // total gate time

    return () => clearTimeout(timer);
  }, []);

  return (
    <AnimatePresence>
      {visible && (
        <motion.div
          className="entry-gate"
          initial={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.6, ease: "linear" }}
        >
          <pre className="gate-text">
ANVESHA
[ INITIALIZING INTELLIGENCE<span className="cursor">_</span> ]
          </pre>
        </motion.div>
      )}
    </AnimatePresence>
  );
}