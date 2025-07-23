export function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const secs = seconds % 60

  if (hours > 0) {
    return `${hours}h ${minutes}m`
  } else if (minutes > 0) {
    return `${minutes}m ${secs}s`
  } else {
    return `${secs}s`
  }
}

export function formatRotPoints(points: number): string {
  if (points >= 0) {
    return `+${points.toFixed(1)}`
  } else {
    return points.toFixed(1)
  }
}

export function formatPercentage(value: number): string {
  return `${value.toFixed(1)}%`
}

export function formatDate(date: string | Date): string {
  const d = typeof date === 'string' ? new Date(date) : date
  return d.toLocaleDateString('en-US', {
    weekday: 'short',
    month: 'short',
    day: 'numeric'
  })
}

export function formatTime(date: string | Date): string {
  const d = typeof date === 'string' ? new Date(date) : date
  return d.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

export function getRotLevelColor(points: number): string {
  if (points < 100) return 'text-healthy-600'
  if (points < 250) return 'text-yellow-600'
  if (points < 400) return 'text-rot-500'
  return 'text-rot-700'
}

export function getRotLevelBg(points: number): string {
  if (points < 100) return 'bg-healthy-50 border-healthy-200'
  if (points < 250) return 'bg-yellow-50 border-yellow-200'
  if (points < 400) return 'bg-rot-50 border-rot-200'
  return 'bg-rot-100 border-rot-300'
}

export function getCategoryColor(category: string): string {
  switch (category.toLowerCase()) {
    case 'junk':
      return 'text-rot-600'
    case 'healthy':
      return 'text-healthy-600'
    case 'neutral':
      return 'text-gray-600'
    default:
      return 'text-gray-600'
  }
}

export function getCategoryBg(category: string): string {
  switch (category.toLowerCase()) {
    case 'junk':
      return 'bg-rot-50 border-rot-200'
    case 'healthy':
      return 'bg-healthy-50 border-healthy-200'
    case 'neutral':
      return 'bg-gray-50 border-gray-200'
    default:
      return 'bg-gray-50 border-gray-200'
  }
}