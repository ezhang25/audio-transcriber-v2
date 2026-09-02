import { Button, Text } from "@chakra-ui/react";

function Dashboard() {
  return (
    <main>
      <h1>This will be a dashboard</h1>

      <Button variant="surface">
        <Text>Start Transcription</Text>
      </Button>
      <Button variant="surface">
        <Text>End Transcription</Text>
      </Button>
    </main>
  );
}

export default Dashboard;