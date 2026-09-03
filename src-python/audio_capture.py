import sounddevice as sd
import numpy as np

class AudioCapture:
    def __init__(self, queue):
        self.queue = queue
        self.buffer = np.array([])
        self.device_index = self.findAudioDevice()

    def findAudioDevice(self):
        devices = sd.query_devices()
        for (i, device) in enumerate(devices):
            if "BlackHole" in device['name']:
                return i
        raise Exception("BlackHole Device not found")

    def audioCallback(self, indata, frames, time, status):
        if status:
            print(status)
        self.buffer = np.append(self.buffer, indata[:, 0])
        if (self.buffer.size > 144000):
            self.queue.put(self.buffer.copy())
            self.buffer = self.buffer[-24000:]

    def start(self):
        device_index = self.device_index
        print(f"Using audio device index: {device_index}")
        self.stream = sd.InputStream(device=device_index, samplerate=48000, channels=1, callback=self.audioCallback)
        self.stream.start()
        return self.stream
