const repos = [
  {
    name: "anvesha-browser",
    desc: "Private search browser runtime",
    link: "https://github.com/your-org/anvesha-browser"
  },
  {
    name: "anvesha-core",
    desc: "Local search, ranking, inference engine (Rust)",
    link: "https://github.com/your-org/anvesha-core"
  },
  {
    name: "anvesha-firewall",
    desc: "Prompt analysis and containment layer",
    link: "https://github.com/your-org/anvesha-firewall"
  },
  {
    name: "anvesha-automation",
    desc: "Autonomous task execution engine",
    link: "https://github.com/your-org/anvesha-automation"
  }
];

export default function Docs() {
  return (
    <section
      style={{
        minHeight: "100vh",
        padding: "12vh 10vw",
        fontFamily: "JetBrains Mono, monospace",
        color: "#f7f7f9",
        position: "relative",
        zIndex: 3
      }}
    >
      <div
        style={{
          fontSize: "1.2rem",
          letterSpacing: "0.18em",
          marginBottom: "2rem"
        }}
      >
        ANVESHA / DOCUMENTATION INDEX
      </div>

      <div
        style={{
          fontSize: "0.85rem",
          lineHeight: 1.8,
          letterSpacing: "0.04em",
          opacity: 0.7,
          maxWidth: "520px",
          marginBottom: "4rem"
        }}
      >
        The following repositories expose
        system components and internal tooling.
        <br />
        <br />
        Access implies responsibility.
      </div>

      <div>
        {repos.map(repo => (
          <a
            key={repo.name}
            href={repo.link}
            target="_blank"
            rel="noreferrer"
            style={{
              display: "block",
              textDecoration: "none",
              color: "#f7f7f9",
              marginBottom: "2rem"
            }}
          >
            <div
              style={{
                letterSpacing: "0.14em",
                marginBottom: "0.4rem"
              }}
            >
              {repo.name}
            </div>

            <div
              style={{
                fontSize: "0.75rem",
                opacity: 0.6
              }}
            >
              {repo.desc}
            </div>
          </a>
        ))}
      </div>
    </section>
  );
}