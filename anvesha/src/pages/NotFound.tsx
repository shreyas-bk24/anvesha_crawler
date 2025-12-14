import { useNavigate } from "react-router-dom";

export default function NotFound() {
  const navigate = useNavigate();

  return (
    <section
      style={{
        minHeight: "100vh",
        display: "flex",
        alignItems: "center",
        paddingLeft: "10vw",
        fontFamily: "JetBrains Mono, monospace",
        color: "#f7f7f9",
        position: "relative",
        zIndex: 3
      }}
    >
      <div>
        <div
          style={{
            fontSize: "1.4rem",
            letterSpacing: "0.18em",
            marginBottom: "2rem"
          }}
        >
          ROUTE NOT FOUND
        </div>

        <div
          style={{
            fontSize: "0.9rem",
            lineHeight: 1.8,
            letterSpacing: "0.04em",
            opacity: 0.7,
            maxWidth: "420px",
            marginBottom: "3rem"
          }}
        >
          The requested endpoint does not exist
          <br />
          or is not exposed to this execution context.
        </div>

        <button
          onClick={() => navigate("/")}
          style={{
            background: "transparent",
            border: "1px solid rgba(255,255,255,0.2)",
            padding: "0.6rem 1.4rem",
            color: "#f7f7f9",
            fontFamily: "inherit",
            letterSpacing: "0.14em",
            cursor: "pointer",
            opacity: 0.8
          }}
        >
          RETURN TO SYSTEM
        </button>
      </div>
    </section>
  );
}