import numpy as np
from faster_whisper import WhisperModel
from scipy.signal import resample_poly
import soundfile as sf


class AudioTranscriber:
    def __init__(self):
        self.model = WhisperModel("medium.en", device="cpu", compute_type="int8")

    def transcribe_pcm(self, pcm_bytes: bytes) -> str:
        samples = np.frombuffer(pcm_bytes, dtype="<f4")

        usable_samples = samples.size - (samples.size % 2)
        if usable_samples == 0:
            return ""

        stereo = samples[:usable_samples].reshape(-1, 2)
        mono_48k = stereo.mean(axis=1, dtype=np.float32)
        mono_16k = resample_poly(mono_48k, up=1, down=3).astype(np.float32)

        segments, _info = self.model.transcribe(
            mono_16k,
            language="en",
            beam_size=1,
            vad_filter=False,
        )
        print(
            f"raw bytes={len(pcm_bytes)}, "
            f"48k mono samples={len(mono_48k)}, "
            f"16k mono samples={len(mono_16k)}, "
            f"peak={np.max(np.abs(mono_48k)):.3f}"
        )   

        sf.write("/tmp/debug_capture.wav", mono_48k, 48_000)

        return "".join(segment.text for segment in segments).strip()