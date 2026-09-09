import asyncio
import websockets
from audio_transcriber import AudioTranscriber
from websocket_server import handle_websocket, send_caption

SAMPLE_RATE = 48_000
CHANNELS = 2
BYTES_PER_SAMPLE = 4
TRANSCRIBE_SECONDS = 8

BYTES_PER_TRANSCRIPTION = (
    SAMPLE_RATE * CHANNELS * BYTES_PER_SAMPLE * TRANSCRIBE_SECONDS
)

async def audio_worker(audio_queue):
    transcriber = AudioTranscriber()
    buffer = bytearray()

    while True:
        pcm_chunk = await audio_queue.get()
        buffer.extend(pcm_chunk)

        while len(buffer) >= BYTES_PER_TRANSCRIPTION:
            audio_to_transcribe = bytes(buffer[:BYTES_PER_TRANSCRIPTION])
            del buffer[:BYTES_PER_TRANSCRIPTION]

            text = await asyncio.to_thread(
                transcriber.transcribe_pcm,
                audio_to_transcribe,
            )

            if text:
                print(f"Transcript: {text}")
                await send_caption(text)

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