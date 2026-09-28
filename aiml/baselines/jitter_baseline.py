import numpy as np

class EWMARttBaseline:
    """Exponential Weighted Moving Average baseline for network RTT smoothing."""
    def __init__(self, alpha: float = 0.2):
        self.alpha = alpha
        self.smoothed_rtt = None

    def update(self, rtt: float) -> float:
        if self.smoothed_rtt is None:
            self.smoothed_rtt = rtt
        else:
            self.smoothed_rtt = self.alpha * rtt + (1 - self.alpha) * self.smoothed_rtt
        return self.smoothed_rtt
