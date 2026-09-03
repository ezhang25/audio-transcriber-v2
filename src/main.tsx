import React from "react";
import ReactDOM from "react-dom/client";

import App from "./App";
import MenuBar from "./components/MenuBar";

import { ChakraProvider, defaultSystem } from "@chakra-ui/react"
import { HStack } from "@chakra-ui/react";
import { HashRouter } from "react-router-dom"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ChakraProvider value={defaultSystem}>
      <HashRouter>
        <HStack h="100dvh" w="100vw" align="stretch" gap="0">
          <MenuBar />
          <App />
        </HStack>
      </HashRouter>
    </ChakraProvider>
  </React.StrictMode>,
);
