import { Button, Text, Center, Box } from "@chakra-ui/react";
import { invoke } from "@tauri-apps/api/core";

function Dashboard() {
  async function startTranscription() {
    try {
      await invoke('startTranscription');
      console.log('Window successfully created!');
    } catch (error) {
      alert(`Failed to open caption window: ${String(error)}`);
    }
  }

  async function endTranscription() {
    try {
      await invoke('endTranscription');
      console.log('Window successfully closed!');
    } catch (error) {
      alert(`Failed to open caption window: ${String(error)}`);
    }
  }

  async function test() {
    try {
      await invoke('pyTest');
      console.log('Hi from Rust!');
    } catch (error) {
      alert(`Message from Rust failed: ${String(error)}`);
    }
  }

  return (
    <main>
      <Box w="full" bg="green">
        <Center w="full" padding="10px">
          <Text fontSize="xl">Dashboard</Text>
        </Center>

        <Center w="full" padding="10px" gap="5">
          <Button variant="surface" onClick={startTranscription}>
            <Text>Start Transcription</Text>
          </Button>
          <Button variant="surface" onClick={endTranscription}>
            <Text>End Transcription</Text>
          </Button>
          <Button variant="surface" onClick={test}>
            <Text>Test</Text>
          </Button>
        </Center>
      </Box>
    </main>
  );
}

export default Dashboard;