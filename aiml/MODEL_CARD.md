# AIML Model Cards: Netcode Intelligence Layer

## Model 1: Input Anomaly Cheat Detector (Tier A)
- **Model Type**: Isolation Forest (Unsupervised)
- **Intended Use**: Real-time detection of aimbot snapping, automated macro input loops, and impossible input rates on raw 60-frame player input sequences.
- **Input Features**: Sliding 60-frame vector $[\Delta x, \Delta y, 	ext{angular velocity}, 	ext{button hold entropy}, 	ext{sub-pixel jitter}]$.
- **Baseline Comparison**: Fixed Threshold Rule Engine (max angular velocity > 720 deg/s).
- **Target Metrics**: F1 Score > 0.94, False Positive Rate < 0.005.
- **Status**: Specified & Scaffolded (**TO BE MEASURED**).
- **Fallback**: Hardcoded threshold rule engine.

## Model 2: Adaptive Jitter Delay Predictor (Tier A)
- **Model Type**: 1D Discrete Kalman Filter + Ridge Regressor
- **Intended Use**: Predicting network RTT variance over upcoming 10-frame horizons to optimize client input delay buffer without inducing user-perceptible lag.
- **Baseline Comparison**: Exponential Weighted Moving Average (EWMA $lpha=0.2$).
- **Target Metrics**: Mean Absolute Error (MAE) < 0.35 frames.
- **Status**: Specified & Scaffolded (**TO BE MEASURED**).
- **Fallback**: Static EWMA + 2 standard deviations safety margin.

## Model 3: LLM Desync Diagnostic Explainer (Tier B)
- **Model Type**: Structured Schema LLM Agent (OpenAI GPT-4o / Claude 3.5 Sonnet)
- **Intended Use**: Forensic root-cause analysis of frame state differential dumps.
- **Safety Policy**: The LLM is NEVER on the simulation or authorization path. Outputs are strictly schema-validated before presentation.
