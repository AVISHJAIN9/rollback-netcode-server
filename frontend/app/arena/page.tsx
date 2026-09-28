import Link from 'next/link'
import { Play, Wifi, Sliders } from 'lucide-react'

export default function ArenaPage() {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-10">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-3xl font-bold text-white mb-1">Deterministic 2D Arena Client</h1>
          <p className="text-slate-400 text-sm">Interactive rollback simulation testbed with network latency and packet loss injection.</p>
        </div>
        <div className="flex gap-3">
          <button className="px-4 py-2 rounded-lg bg-primary hover:bg-primary/90 text-black font-semibold text-sm transition-all">
            Find Match
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
        {/* Arena Viewport */}
        <div className="lg:col-span-3 aspect-video bg-slate-950 rounded-2xl border border-slate-800 relative flex items-center justify-center overflow-hidden">
          <div className="text-center">
            <Play className="w-12 h-12 text-slate-700 mx-auto mb-3" />
            <div className="text-slate-400 font-mono text-sm">Interactive 2D Arena Canvas</div>
            <div className="text-slate-600 text-xs mt-1">Simulating 60 FPS Fixed-Point Kinematics</div>
          </div>
        </div>

        {/* Network Impairment Controls */}
        <div className="p-6 rounded-2xl bg-surface border border-slate-800 space-y-6">
          <h3 className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
            <Sliders className="w-4 h-4 text-primary" /> Network Impairment
          </h3>
          <div className="space-y-4">
            <div>
              <div className="flex justify-between text-xs font-mono text-slate-400 mb-1.5">
                <span>Artificial Latency</span>
                <span className="text-white">50 ms</span>
              </div>
              <input type="range" min="0" max="300" defaultValue="50" className="w-full accent-primary" />
            </div>
            <div>
              <div className="flex justify-between text-xs font-mono text-slate-400 mb-1.5">
                <span>Packet Loss</span>
                <span className="text-white">5 %</span>
              </div>
              <input type="range" min="0" max="50" defaultValue="5" className="w-full accent-primary" />
            </div>
            <div>
              <div className="flex justify-between text-xs font-mono text-slate-400 mb-1.5">
                <span>Jitter Variance</span>
                <span className="text-white">15 ms</span>
              </div>
              <input type="range" min="0" max="100" defaultValue="15" className="w-full accent-primary" />
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
