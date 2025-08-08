import { WaitlistFormData } from '@/types'
import { NextResponse } from 'next/server'

export async function POST(request: Request) {
  try {
    const formData: WaitlistFormData = await request.json()
    
    // Basic validation
    if (!formData.email || !formData.userType || !formData.interest) {
      return NextResponse.json(
        { error: 'Missing required fields' },
        { status: 400 }
      )
    }
    console.log('Received waitlist form data:', formData)
    // Here you would integrate with your preferred service:
    // - Airtable
    // - Notion Database
    // - Mailchimp
    // - ConvertKit
    // - Supabase
    // - Firebase
    
    // Example with Airtable:
    // await saveToAirtable(formData)
    
    // Example with email service:
    // await sendWelcomeEmail(formData.email)
    await new Promise(resolve => setTimeout(resolve, 5000)) // Simulate async operation
    
    return NextResponse.json(
      { message: 'Subscription successful' },
      { status: 200 }
    )
  } catch (error) {
    console.error('Error in waitlist:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}