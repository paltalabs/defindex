import { NextResponse } from 'next/server';

interface WaitlistFormData {
  email: string;
  userType: string;
  interest: string;
  name?: string;
  company?: string;
  telegram?: string;
}

export async function POST(request: Request) {
  try {
    const formData: WaitlistFormData = await request.json();

    if (!formData.email || !formData.userType || !formData.interest) {
      return NextResponse.json(
        { error: 'Missing required fields' },
        { status: 400 }
      );
    }

    // Validate userType
    const allowedUserTypes = ['individual', 'partner', 'developer'];
    if (
      typeof formData.userType !== 'string' ||
      formData.userType.length > 50 ||
      !allowedUserTypes.includes(formData.userType)
    ) {
      return NextResponse.json(
        { error: 'Invalid userType' },
        { status: 400 }
      );
    }

    // Validate interest length to prevent abuse
    if (
      typeof formData.interest !== 'string' ||
      formData.interest.length > 500
    ) {
      return NextResponse.json(
        { error: 'Invalid interest' },
        { status: 400 }
      );
    }

    // Validate email format
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(formData.email)) {
      return NextResponse.json(
        { error: 'Invalid email format' },
        { status: 400 }
      );
    }

    const url = process.env.N8N_WEBHOOK_URL;
    if (!url) {
      console.error('N8N_WEBHOOK_URL environment variable is not set');
      return NextResponse.json(
        { error: 'Internal server error' },
        { status: 500 }
      );
    }
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-from-site': 'DeFindex',
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
    console.error('Error in waitlist:', error);
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}
