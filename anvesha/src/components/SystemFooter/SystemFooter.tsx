export default function SystemFooter() {
  return (
    <footer
      style={{
        position: "relative",
        minHeight: "40vh",
        padding: "10vh 10vw",
        fontFamily: "JetBrains Mono, monospace",
        color: "#8a8f98",
        zIndex: 3
      }}
    >
      <div
        style={{
          borderTop: "1px solid rgba(255,255,255,0.08)",
          paddingTop: "4vh"
        }}
      >
        <div
          style={{
            fontSize: "1rem",
            letterSpacing: "0.18em",
            color: "#f7f7f9",
            marginBottom: "1.5rem"
          }}
        >
          ANVESHA
        </div>

        <div
          style={{
            fontSize: "0.75rem",
            letterSpacing: "0.14em",
            lineHeight: 1.8,
            opacity: 0.7,
            marginBottom: "2.5rem"
          }}
        >
          PRIVATE INTELLIGENCE SYSTEM<br />
          <br />
          LOCAL BY DESIGN<br />
          BUILT IN RUST
        </div>
          <div
  style={{
    display: "flex",
    gap: "1.5rem",
    fontSize: "0.65rem",
    letterSpacing: "0.16em",
    opacity: 0.5,
    marginBottom: "2rem"
  }}
>
  <a
    href="/docs"
    style={{
      color: "#8a8f98",
      textDecoration: "none"
    }}
  >
    DOCS
  </a>

  <a
    href="https://github.com/your-org"
    target="_blank"
    rel="noreferrer"
    style={{
      color: "#8a8f98",
      textDecoration: "none"
    }}
  >
    REPOSITORIES
  </a>
</div>
        <div
          style={{
            fontSize: "0.65rem",
            letterSpacing: "0.16em",
            opacity: 0.4
          }}
        >
          © 2025
        </div>
      </div>
    </footer>
  );
}