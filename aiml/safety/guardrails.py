class MLSafetyGuardrails:
  """Enforces hard bounds, deterministic fallback, and kill-switch logic for AI models."""
  def __init__(self):
      self.kill_switch_active = False
      self.min_delay_frames = 1
      self.max_delay_frames = 5
      self.max_rate_change_per_min = 1

  def sanitize_input_delay(self, model_proposed_delay: float, current_delay: int) -> int:
      if self.kill_switch_active or model_proposed_delay is None:
          # Fallback: Static 2-frame delay
          return 2
      
      # 1. Clamp within hard safety envelope [1, 5] frames
      bounded = max(self.min_delay_frames, min(self.max_delay_frames, int(round(model_proposed_delay))))
      
      # 2. Rate-limit changes to prevent oscillation
      if abs(bounded - current_delay) > self.max_rate_change_per_min:
          return current_delay + (1 if bounded > current_delay else -1)
      
      return bounded
