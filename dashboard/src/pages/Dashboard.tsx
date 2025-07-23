import React, { useEffect, useState } from 'react'
import { 
  Brain, 
  Clock, 
  TrendingUp, 
  Zap,
  Eye,
  Target,
  Calendar,
  Activity
} from 'lucide-react'
import RotMeter from '../components/RotMeter'
import ActivityChart from '../components/ActivityChart'
import TopOffenders from '../components/TopOffenders'
import QuickActions from '../components/QuickActions'
import { useDailyStats } from '../hooks/useDailyStats'
import { formatDuration, formatRotPoints } from '../utils/format'

const Dashboard: React.FC = () => {
  const { stats, loading, error, refetch } = useDailyStats()
  const [currentTime, setCurrentTime] = useState(new Date())

  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date())
    }, 1000)

    return () => clearInterval(timer)
  }, [])

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600"></div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="text-center py-12">
        <div className="text-rot-600 mb-4">
          <Brain className="w-12 h-12 mx-auto" />
        </div>
        <h3 className="text-lg font-medium text-gray-900 mb-2">Failed to load dashboard</h3>
        <p className="text-gray-500 mb-4">{error}</p>
        <button onClick={refetch} className="btn-primary">
          Try Again
        </button>
      </div>
    )
  }

  const rotLevel = stats ? getRotLevel(stats.total_rot_points) : 'low'
  const rotPercentage = stats ? Math.min((stats.total_rot_points / 500) * 100, 100) : 0

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="text-center">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Welcome back! 👋
        </h1>
        <p className="text-gray-600">
          {currentTime.toLocaleTimeString()} • Let's see how you're doing today
        </p>
      </div>

      {/* Main Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {/* Rot Meter */}
        <div className="lg:col-span-2">
          <div className="card text-center">
            <h3 className="text-lg font-semibold text-gray-900 mb-4">Today's Rot Score</h3>
            <RotMeter 
              value={stats?.total_rot_points || 0}
              max={500}
              size={200}
              level={rotLevel}
            />
            <div className="mt-4">
              <p className="text-2xl font-bold text-gray-900">
                {formatRotPoints(stats?.total_rot_points || 0)}
              </p>
              <p className="text-sm text-gray-500">
                {rotLevel === 'low' && '🌱 Doing great!'}
                {rotLevel === 'medium' && '⚠️ Watch out!'}
                {rotLevel === 'high' && '🔥 Time to touch grass!'}
                {rotLevel === 'critical' && '🚨 Brain rot overload!'}
              </p>
            </div>
          </div>
        </div>

        {/* Quick Stats */}
        <div className="space-y-6">
          <div className="card">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Screen Time</p>
                <p className="text-2xl font-bold text-gray-900">
                  {formatDuration((stats?.total_duration_minutes || 0) * 60)}
                </p>
              </div>
              <Clock className="w-8 h-8 text-primary-600" />
            </div>
          </div>

          <div className="card">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Trend</p>
                <p className="text-2xl font-bold text-healthy-600">
                  -12%
                </p>
                <p className="text-xs text-gray-500">vs yesterday</p>
              </div>
              <TrendingUp className="w-8 h-8 text-healthy-600" />
            </div>
          </div>
        </div>

        {/* Status Cards */}
        <div className="space-y-6">
          <div className="card">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Focus Score</p>
                <p className="text-2xl font-bold text-primary-600">
                  {Math.max(0, 100 - Math.round(rotPercentage))}%
                </p>
              </div>
              <Target className="w-8 h-8 text-primary-600" />
            </div>
          </div>

          <div className="card">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm font-medium text-gray-600">Streak</p>
                <p className="text-2xl font-bold text-yellow-600">
                  3 days
                </p>
              </div>
              <Zap className="w-8 h-8 text-yellow-600" />
            </div>
          </div>
        </div>
      </div>

      {/* Charts and Details */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* Activity Timeline */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 mb-4 flex items-center">
            <Activity className="w-5 h-5 mr-2" />
            Today's Activity
          </h3>
          <ActivityChart data={stats?.breakdown || []} />
        </div>

        {/* Top Offenders */}
        <div className="card">
          <h3 className="text-lg font-semibold text-gray-900 mb-4 flex items-center">
            <Eye className="w-5 h-5 mr-2" />
            Biggest Time Wasters
          </h3>
          <TopOffenders apps={stats?.top_offenders || []} />
        </div>
      </div>

      {/* Quick Actions */}
      <QuickActions onRefresh={refetch} />
    </div>
  )
}

function getRotLevel(rotPoints: number): 'low' | 'medium' | 'high' | 'critical' {
  if (rotPoints < 100) return 'low'
  if (rotPoints < 250) return 'medium'
  if (rotPoints < 400) return 'high'
  return 'critical'
}

export default Dashboard