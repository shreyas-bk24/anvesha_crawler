import { Routes, Route } from "react-router-dom";
import Docs from "./pages/Docs";
import NotFound from "./pages/NotFound";
import Anvesha from "./routes/Anvesha";

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Anvesha />} />
      <Route path="/docs" element={<Docs />} />
      <Route path="*" element={<NotFound />} />
    </Routes>
  );
}