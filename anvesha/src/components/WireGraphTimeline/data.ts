export const nodes = [
  {
    id: "index",
    x: 20,
    y: 15,
    label: "2024 Q4 — Index bootstrap",
    visible: true
  },
  {
    id: "ranking",
    x: 45,
    y: 30,
    label: "2025 Q1 — Local ranking",
    visible: true
  },
  {
    id: "ranking-exp",
    x: 55,
    y: 22,
    label: "2025 Q1 — Alt ranking path",
    visible: false
  },
  {
    id: "firewall",
    x: 35,
    y: 45,
    label: "2025 Q1 — Firewall init",
    visible: true
  },
  {
    id: "sandbox",
    x: 25,
    y: 55,
    label: "████ — Isolation sandbox",
    visible: false
  },
  {
    id: "tasks",
    x: 65,
    y: 55,
    label: "████ Q2 — Task engine",
    visible: false
  },
  {
    id: "protocol",
    x: 50,
    y: 75,
    label: "████ — █████ Protocol",
    visible: false
  }
];

export const edges: {
  from: string;
  to: string;
  type: "main" | "branch";
}[] = [
  { from: "index", to: "ranking", type: "main" },
  { from: "ranking", to: "firewall", type: "main" },
  { from: "firewall", to: "tasks", type: "main" },
  { from: "tasks", to: "protocol", type: "main" },

  // branches
  { from: "ranking", to: "ranking-exp", type: "branch" },
  { from: "firewall", to: "sandbox", type: "branch" },
  { from: "sandbox", to: "tasks", type: "branch" }
];