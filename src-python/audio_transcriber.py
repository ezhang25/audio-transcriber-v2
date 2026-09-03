import faster_whisper
import librosa

class AudioTranscriber:
    def __init__(self):
        self.model = faster_whisper.WhisperModel("medium", device="cpu")

    def transcribe_audio(self, file_path):
        chunk_16k = librosa.resample(file_path, orig_sr=48000, target_sr=16000)
        segments, info = self.model.transcribe(chunk_16k, language="en")
        text = ""
        for segment in segments:
            text += segment.text
        return text
