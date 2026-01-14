'use client'

import React from "react"

import posthog from 'posthog-js'
import { PostHogProvider as PHProvider } from 'posthog-js/react'

// Initialize PostHog once on module load
if (typeof window !== 'undefined' && !posthog.__loaded) {
  posthog.init(process.env.NEXT_PUBLIC_POSTHOG_KEY as string, {
    api_host: '/relay-It6G',
    ui_host: 'https://us.i.posthog.com',
    person_profiles: 'identified_only',
    capture_pageview: true,
    capture_pageleave: true,
    capture_exceptions: true,
    debug: process.env.NODE_ENV === 'development',
    loaded: () => {
      if (process.env.NODE_ENV === 'development') {
        console.log('✅ PostHog loaded successfully')
      }
    }
  })
}

export function PostHogProvider({ children }: { children: React.ReactNode }) {
  return (
    <PHProvider client={posthog}>
      {/* eslint-disable-next-line @typescript-eslint/no-explicit-any */}
      {children as any}
    </PHProvider>
  )
}