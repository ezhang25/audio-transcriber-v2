import { Routes, Route } from "react-router-dom";

import Home from "./pages/Home";
import Dashboard from "./pages/Dashboard";
import TranscriptLogs from "./pages/TranscriptLogs";
import Settings from "./pages/Settings";

function App() {
  return (
    <main className="container">
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/dashboard" element={<Dashboard />}/>
          <Route path="/transcript" element={<TranscriptLogs />}/>
          <Route path="/settings" element={<Settings />}/>
        </Routes>
    </main>
  );
}

export default App;
