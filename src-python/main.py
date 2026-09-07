import asyncio
import websockets
from audio_capture import AudioCapture
from audio_transcriber import AudioTranscriber
from websocket_server import handle_websocket

async def audio_worker(audio_queue):
    while True:
        pcm_chunk = await audio_queue.get()
        print(f"Worker received {len(pcm_chunk)} PCM bytes")

async def main():
    audio_queue = asyncio.Queue()

    async def websocket_handler(websocket):
        await handle_websocket(websocket, audio_queue)

    server = await websockets.serve(websocket_handler, "127.0.0.1", 8765)

    asyncio.create_task(audio_worker(audio_queue))

    print("WebSocket server started on ws://localhost:8765")
    await server.serve_forever()
    

if __name__ == "__main__":
    asyncio.run(main())