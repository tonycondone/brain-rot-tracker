import React from 'react'

interface RotMeterProps {
  value: number
  max: number
  size?: number
  level?: 'low' | 'medium' | 'high' | 'critical'
}

const RotMeter: React.FC<RotMeterProps> = ({ 
  value, 
  max, 
  size = 120, 
  level = 'low' 
}) => {
  const percentage = Math.min((value / max) * 100, 100)
  const circumference = 2 * Math.PI * 45 // radius of 45
  const strokeDasharray = circumference
  const strokeDashoffset = circumference - (percentage / 100) * circumference

  const getLevelColor = () => {
    switch (level) {
      case 'low': return '#22c55e'
      case 'medium': return '#eab308'
      case 'high': return '#ef4444'
      case 'critical': return '#dc2626'
      default: return '#22c55e'
    }
  }

  const getLevelEmoji = () => {
    switch (level) {
      case 'low': return '🌱'
      case 'medium': return '⚠️'
      case 'high': return '🔥'
      case 'critical': return '🚨'
      default: return '🌱'
    }
  }

  return (
    <div className="rot-meter" style={{ width: size, height: size }}>
      <svg
        width={size}
        height={size}
        viewBox="0 0 100 100"
        className="rot-ring"
      >
        {/* Background circle */}
        <circle
          cx="50"
          cy="50"
          r="45"
          fill="none"
          stroke="#e5e7eb"
          strokeWidth="8"
          className="rot-ring-bg"
        />
        
        {/* Progress circle */}
        <circle
          cx="50"
          cy="50"
          r="45"
          fill="none"
          stroke={getLevelColor()}
          strokeWidth="8"
          strokeLinecap="round"
          strokeDasharray={strokeDasharray}
          strokeDashoffset={strokeDashoffset}
          className="rot-ring-progress"
          style={{
            transition: 'stroke-dashoffset 0.5s ease-out, stroke 0.3s ease-out'
          }}
        />
      </svg>
      
      {/* Center content */}
      <div className="absolute inset-0 flex flex-col items-center justify-center">
        <div className="text-2xl mb-1">{getLevelEmoji()}</div>
        <div className="text-lg font-bold text-gray-900">
          {Math.round(percentage)}%
        </div>
        <div className="text-xs text-gray-500">
          {value.toFixed(1)} / {max}
        </div>
      </div>
    </div>
  )
}

export default RotMeter