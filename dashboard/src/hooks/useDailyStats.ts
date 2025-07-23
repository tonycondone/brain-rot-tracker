import { useState, useEffect } from 'react'

interface CategoryBreakdown {
  category: string
  rot_points: number
  duration_minutes: number
  percentage: number
}

interface AppBreakdown {
  app_or_site: string
  rot_points: number
  duration_minutes: number
  category: string
}

interface DailyStats {
  date: string
  total_rot_points: number
  total_duration_minutes: number
  breakdown: CategoryBreakdown[]
  top_offenders: AppBreakdown[]
}

interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

export function useDailyStats(date?: string) {
  const [stats, setStats] = useState<DailyStats | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchStats = async () => {
    try {
      setLoading(true)
      setError(null)

      const params = new URLSearchParams()
      if (date) params.append('date', date)

      const response = await fetch(`/api/rot/daily?${params}`)
      
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`)
      }

      const result: ApiResponse<DailyStats> = await response.json()
      
      if (result.success && result.data) {
        setStats(result.data)
      } else {
        throw new Error(result.error || 'Failed to fetch daily stats')
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unknown error occurred')
      console.error('Error fetching daily stats:', err)
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchStats()
  }, [date])

  return {
    stats,
    loading,
    error,
    refetch: fetchStats
  }
}