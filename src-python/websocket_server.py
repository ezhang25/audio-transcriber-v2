import asyncio
import websockets

websocket_servers = set()

async def send_caption(message):
    global websocket_servers
    print("Caption sent")
    disconnected = set()
    for websocket in websocket_servers.copy():
        try:
            await websocket.send(message)
        except:
            disconnected.add(websocket)
    websocket_servers -= disconnected

async def handle_websocket(websocket):
    websocket_servers.add(websocket)
    print("Client connected")

    await websocket.send("Hello from Python")

    try:
        await websocket.wait_closed()
    finally:
        websocket_servers.remove(websocket)
        print("Client disconnected")

async def main():
    server = await websockets.serve(handle_websocket, 'localhost', 1111)
    await server.wait_closed()

if __name__ == "__main__":
    asyncio.run(main())