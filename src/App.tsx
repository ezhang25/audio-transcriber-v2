import { Routes, Route } from "react-router-dom";
import { Box } from "@chakra-ui/react";

import Home from "./pages/Home";
import Dashboard from "./pages/Dashboard";
import TranscriptLogs from "./pages/TranscriptLogs";
import Settings from "./pages/Settings";

function App() {
  return (
    <Box flex="1" h="full" minW="0">
      <Routes>          
        <Route path="/" element={<Home />} />
        <Route path="/dashboard" element={<Dashboard />}/>
        <Route path="/transcript" element={<TranscriptLogs />}/>
        <Route path="/settings" element={<Settings />}/>
      </Routes>
    </Box>
  );
}

export default App;
