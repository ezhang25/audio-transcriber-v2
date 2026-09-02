import { Box, Text, Center, Flex, Button, Separator } from "@chakra-ui/react";
import { Link } from "react-router-dom";

function MenuBar() {
  return(
    <Box w="200px">
      <Flex padding="2%" h="full" w="full" flexDir="column" bg="lightgrey">
        <Box w="full" justifyContent="center">
          <Center>
            <Text fontSize="lg" fontWeight="bolder" textWrap="nowrap" color="black" padding="10%">
              AudioTranscriber v2
            </Text>
          </Center>
        </Box>

        <Separator borderWidth="1px" borderColor="black"/>

        <Flex padding="2%" h="200px" w="full" flexDir="column" justify="space-evenly">
          <Box bg="lightgrey" w="full">
            <Link to="/">
              <Button variant="ghost" w="full">
                <Text>                    
                  Home
                </Text>
              </Button>
            </Link>
          </Box>
          <Box>
            <Link to="/dashboard">
              <Button variant="ghost" w="full">
                <Text>
                  Dashboard
                </Text>             
              </Button>
            </Link>
          </Box>
          <Box>
            <Link to="/transcript">
              <Button variant="ghost" w="full">
                <Text>
                  Transcript Logs
                </Text>
              </Button>
            </Link>
          </Box>
          <Box>
            <Link to="/settings">
              <Button variant="ghost" w="full">
                <Text>
                  Settings
                </Text>
              </Button>
            </Link>
          </Box>
        </Flex>
     </Flex>
    </Box>
  );
}

export default MenuBar;