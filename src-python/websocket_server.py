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

async def handle_websocket(websocket, audio_queue):
    websocket_servers.add(websocket)
    print("Client connected")

    await websocket.send("Hello from Python")

    try:   
        async for message in websocket:
            if isinstance(message, bytes):
                await audio_queue.put(message)
            else:
                print(f"Python received text: {message}")

    except websockets.exceptions.ConnectionClosed:
        pass

    finally:
        websocket_servers.discard(websocket)
        print("Client disconnected")

async def main():
    server = await websockets.serve(handle_websocket, 'localhost', 1111)
    await server.wait_closed()

if __name__ == "__main__":
    asyncio.run(main())