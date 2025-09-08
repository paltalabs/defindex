
import { NextResponse } from 'next/server'
interface WaitlistFormData {
  email: string;
  userType: string;
  interest: string;
}

export async function POST(request: Request) {
  try {
    const formData: WaitlistFormData = await request.json()
    
    if (!formData.email || !formData.userType || !formData.interest) {
      return NextResponse.json(
        { error: 'Missing required fields' },
        { status: 400 }
      )
    }
    
    const url = "https://n8n.srv914453.hstgr.cloud/webhook/fc686436-182c-40c6-bc0e-370d315cd604"
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-from-site': 'DeFindex'
      },
      body: JSON.stringify(formData),
    });

    if (!response.ok) {
      console.error('Failed to post data to webhook:', await response.text());
      return NextResponse.json(
        { error: 'Failed to process subscription' },
        { status: 500 }
      );
    }
    
    const responseData = await response.json();
    return NextResponse.json(
      { message: 'Subscription successful', data: responseData },
      { status: 200 }
    );
  } catch (error) {
    console.error('Error in waitlist:', error)
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    )
  }
}