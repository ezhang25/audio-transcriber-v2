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
      <Box w="full">
        <Center w="full" padding="20px" bg="grey">
          <Text fontSize="xl" fontWeight="bold" color="white">Dashboard</Text>
        </Center>

        <Center w="full" h="full" padding="50px" gap="20px">
          <Button variant="surface" onClick={startTranscription}>
            <Text>Start Transcription</Text>
          </Button>
          <Button variant="surface" onClick={endTranscription}>
            <Text>End Transcription</Text>
          </Button>
        </Center>
      </Box>
    </main>
  );
}

export default Dashboard;