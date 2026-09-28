import numpy as np

class KalmanJitterPredictor:
    """Tier A 1D Kalman Filter estimating true network transit latency and variance."""
    def __init__(self, process_noise: float = 1e-3, measurement_noise: float = 1e-1):
        self.x = 0.0 # Estimated RTT
        self.p = 1.0 # Error covariance
        self.q = process_noise
        self.r = measurement_noise

    def step(self, measurement: float) -> float:
        # Prediction
        p_pred = self.p + self.q
        
        # Update
        k = p_pred / (p_pred + self.r)
        self.x = self.x + k * (measurement - self.x)
        self.p = (1 - k) * p_pred
        return self.x
