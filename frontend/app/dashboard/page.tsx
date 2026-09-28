import { Activity, Clock, Shield, RefreshCw } from 'lucide-react'

export default function DashboardPage() {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-white mb-2">Netcode Internals & Telemetry</h1>
        <p className="text-slate-400 text-sm">Real-time inspection of the 128-frame snapshot ring buffer, rollback frequency, and clock sync state.</p>
      </div>

      {/* KPI Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
        <div className="p-6 rounded-xl bg-surface border border-slate-800">
          <div className="text-slate-400 text-xs font-mono mb-1">SIMULATION TICK RATE</div>
          <div className="text-2xl font-bold text-white">60.0 Hz</div>
          <div className="text-xs text-success mt-2">16.666 ms fixed interval</div>
        </div>
        <div className="p-6 rounded-xl bg-surface border border-slate-800">
          <div className="text-slate-400 text-xs font-mono mb-1">ROLLBACK FREQUENCY</div>
          <div className="text-2xl font-bold text-primary">TO BE MEASURED</div>
          <div className="text-xs text-slate-400 mt-2">Under active game traffic</div>
        </div>
        <div className="p-6 rounded-xl bg-surface border border-slate-800">
          <div className="text-slate-400 text-xs font-mono mb-1">ESTIMATED NETWORK RTT</div>
          <div className="text-2xl font-bold text-warning">TO BE MEASURED</div>
          <div className="text-xs text-slate-400 mt-2">4-probe NTP estimator</div>
        </div>
        <div className="p-6 rounded-xl bg-surface border border-slate-800">
          <div className="text-slate-400 text-xs font-mono mb-1">DESYNC INCIDENTS</div>
          <div className="text-2xl font-bold text-success">0 Detected</div>
          <div className="text-xs text-success mt-2">100% Checksum agreement</div>
        </div>
      </div>

      {/* Ring Buffer SVG Visualizer Mock */}
      <div className="p-8 rounded-2xl bg-surface border border-slate-800 mb-8">
        <h3 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <RefreshCw className="w-5 h-5 text-primary" /> Snapshot Ring Buffer State (128 Slots)
        </h3>
        <div className="grid grid-cols-16 sm:grid-cols-32 gap-1.5">
          {Array.from({ length: 64 }).map((_, idx) => (
            <div 
              key={idx} 
              className={`h-8 rounded-sm ${idx < 48 ? 'bg-emerald-500/80' : idx < 58 ? 'bg-primary/80' : 'bg-slate-700/50'} flex items-center justify-center text-[10px] font-mono text-black font-semibold`}
            >
              {idx}
            </div>
          ))}
        </div>
        <div className="flex gap-6 mt-4 text-xs font-mono text-slate-400">
          <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded-sm bg-emerald-500"></span> Confirmed Authoritative</span>
          <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded-sm bg-primary"></span> Predicted Forward</span>
          <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded-sm bg-slate-700"></span> Empty / Recycled</span>
        </div>
      </div>
    </div>
  )
}
