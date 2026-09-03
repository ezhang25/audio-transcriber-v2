import asyncio
import websockets
from websocket_server import send_caption, handle_websocket


async def main():
    server = await websockets.serve(handle_websocket, "127.0.0.1", 8765)
    print("WebSocket server started on ws://localhost:8765")
    await server.serve_forever()

if __name__ == "__main__":
    asyncio.run(main())