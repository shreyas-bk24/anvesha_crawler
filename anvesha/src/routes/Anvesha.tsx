import EntryGate from "../components/EntryGate/EntryGate";
import Manifesto from "../components/Manifesto/Manifesto";
import SystemArchitecture from "../SystemArchitecture/SystemArchitecture";
import SystemTransition from "../components/SystemTransition/SystemTransition";
import BrokenGrid from "../components/BrokenGrid/BrokenGrid";
import AIFirewall from "../components/AIFirewall/AIFirewall";
import AutomationRules from "../components/AutomationRules/AutomationRules"
import WireGraphTimeline from "../components/WireGraphTimeline/WireGraphTimeline";
import SystemFooter from "../components/SystemFooter/SystemFooter";

export default function Anvesha() {
  return (
    <>
      <EntryGate />
      <Manifesto />
      <SystemArchitecture />
      <SystemTransition />
      <BrokenGrid />
      <AIFirewall />
      <AutomationRules />
      <WireGraphTimeline />
      <SystemFooter />  
    </>
  );
}
