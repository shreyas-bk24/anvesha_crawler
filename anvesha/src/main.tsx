import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter } from "react-router-dom";


import "./styles/globals.css";   // ✅ REQUIRED
import "./styles/reset.css";     // ✅ REQUIRED
import "./styles/typography.css";



import "./styles/globals.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>
);