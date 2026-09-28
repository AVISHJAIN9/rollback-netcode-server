class ModelRegistry:
  """Manages model versions, staged promotions, and instant rollbacks (<5 minutes)."""
  def __init__(self):
      self.models = {
          "cheat_detector": {"v1.0": "RuleEngine_v1", "v1.1": "IsolationForest_v1.1"},
          "jitter_predictor": {"v1.0": "EWMA_v1", "v1.1": "KalmanFilter_v1.1"}
      }
      self.active_versions = {
          "cheat_detector": "v1.1",
          "jitter_predictor": "v1.1"
      }

  def rollback_model(self, model_name: str, target_version: str) -> bool:
      if model_name in self.models and target_version in self.models[model_name]:
          self.active_versions[model_name] = target_version
          return True
      return False
