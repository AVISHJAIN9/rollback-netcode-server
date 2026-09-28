import numpy as np
from sklearn.ensemble import IsolationForest

class CheatDetectorModel:
    """Tier A Isolation Forest model for player input anomaly detection."""
    def __init__(self, contamination: float = 0.01):
        self.model = IsolationForest(
            n_estimators=100,
            contamination=contamination,
            random_state=42
        )
        self.is_trained = False

    def fit(self, X: np.ndarray):
        """Fits the Isolation Forest on normal human player input telemetry."""
        self.model.fit(X)
        self.is_trained = True

    def predict_anomaly_score(self, X: np.ndarray) -> np.ndarray:
        if not self.is_trained:
            raise RuntimeError("Model must be trained before predicting anomaly scores.")
        return -self.model.score_samples(X)
