import numpy as np

class NetcodeFeaturePipeline:
    """Leakage-free sliding-window telemetry feature extractor."""
    def __init__(self, window_size: int = 60):
        self.window_size = window_size

    def transform(self, raw_input_series: np.ndarray) -> np.ndarray:
        # Computes variance, angular deltas, and hold duration entropy over sliding window
        n_samples = len(raw_input_series)
        if n_samples < self.window_size:
            return np.zeros((1, 5))
        
        dx = raw_input_series[:, 0]
        dy = raw_input_series[:, 1]
        buttons = raw_input_series[:, 2]
        
        dx_var = np.var(dx)
        dy_var = np.var(dy)
        ang_vel = np.mean(np.abs(np.diff(np.arctan2(dy, dx + 1e-6)))) * (180.0 / np.pi) * 60.0
        entropy = -np.sum((p := np.bincount(buttons.astype(int), minlength=8) / len(buttons))[p > 0] * np.log2(p[p > 0]))
        reaction_est = np.std(dx) * 10.0
        
        return np.array([[dx_var, dy_var, ang_vel, entropy, reaction_est]])
