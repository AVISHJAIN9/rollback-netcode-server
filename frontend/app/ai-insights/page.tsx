import { Sparkles, ShieldAlert, Cpu, AlertTriangle } from 'lucide-react'

export default function AIInsightsPage() {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
      <div className="mb-8">
        <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-xs font-mono text-purple-400 mb-3">
          <Sparkles className="w-3.5 h-3.5" /> AIML TELEMETRY & DIAGNOSTICS
        </div>
        <h1 className="text-3xl font-bold text-white mb-2">AI Diagnostics & Explainer Studio</h1>
        <p className="text-slate-400 text-sm">Model performance metrics, cheat detection anomaly scoring, and LLM-assisted desync forensic analysis.</p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
        {/* Tier A: Cheat Detection */}
        <div className="p-6 rounded-2xl bg-surface border border-slate-800">
          <h3 className="text-lg font-bold text-white mb-3 flex items-center gap-2">
            <ShieldAlert className="w-5 h-5 text-accent" /> Tier A: Isolation Forest Cheat Detector
          </h3>
          <p className="text-slate-400 text-sm mb-4">
            Evaluates raw 60-frame input vectors for macro looping, sub-human reaction latencies, and unnatural angular snapping.
          </p>
          <div className="space-y-3 font-mono text-xs">
            <div className="flex justify-between p-3 rounded bg-slate-900 border border-slate-800">
              <span className="text-slate-400">Target F1 Score:</span>
              <span className="text-primary">&gt; 0.94</span>
            </div>
            <div className="flex justify-between p-3 rounded bg-slate-900 border border-slate-800">
              <span className="text-slate-400">Measured F1 Score:</span>
              <span className="text-amber-400 font-bold">TO BE MEASURED</span>
            </div>
            <div className="flex justify-between p-3 rounded bg-slate-900 border border-slate-800">
              <span className="text-slate-400">False Positive Rate:</span>
              <span className="text-amber-400 font-bold">TO BE MEASURED</span>
            </div>
          </div>
        </div>

        {/* Tier B: LLM Desync Explainer */}
        <div className="p-6 rounded-2xl bg-surface border border-slate-800">
          <h3 className="text-lg font-bold text-white mb-3 flex items-center gap-2">
            <Cpu className="w-5 h-5 text-primary" /> Tier B: LLM Desync Explainer
          </h3>
          <p className="text-slate-400 text-sm mb-4">
            Parses binary state differential dumps between client and server, generating plain-English causality reports.
          </p>
          <div className="p-4 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-slate-300 space-y-2">
            <div className="text-slate-500">// Sample Diagnostic Output</div>
            <div><span className="text-accent">Root Cause:</span> Fixed-point rounding discrepancy on player velocity during AABB corner collision.</div>
            <div><span className="text-accent">Affected Frame:</span> #442</div>
            <div><span className="text-accent">Status:</span> Resolved via snapshot resimulation.</div>
          </div>
        </div>
      </div>
    </div>
  )
}
