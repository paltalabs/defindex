import 'dotenv/config';
import { NextResponse } from 'next/server';
export async function GET() {
  try {

    const apiKey = process.env.DEFINDEX_API_KEY;
    const vaultId = process.env.DEFINDEX_VAULT_ID || 'CBNKCU3HGFKHFOF7JTGXQCNKE3G3DXS5RDBQUKQMIIECYKXPIOUGB2S3'; // Beans USDC vault
    if (!apiKey) {
      throw new Error('Missing DEFINDEX_API_KEY environment variable');
    }

    const apiUrl = `https://api.defindex.io/vault/${vaultId}/apy?network=mainnet`;
    const response = await fetch(apiUrl, {
      headers: {
        'Authorization': `Bearer ${apiKey}`
      }
    });
    if (!response.ok) {
      console.error('Error response from Defindex API:', response.status, response.statusText);
      throw new Error(`Error fetching APY: ${response.statusText}`);
    }
    const data = await response.json();
    const apy = data.apy;
    if (typeof apy !== 'number') {
      console.error('Invalid APY value:', data);
      throw new Error('Invalid APY value received from API');
    }
    
    
    return NextResponse.json({
      apy: parseFloat(apy.toFixed(2)),
      timestamp: new Date().toISOString(),
      source: 'defindex-api'
    }, {
      status: 200,
      headers: {
        'Cache-Control': 'public, s-maxage=300, stale-while-revalidate=600', // Cache for 5 minutes
      }
    });
    
  } catch (error) {
    console.error('Error fetching APY:', error);
    
    // Return fallback APY in case of error
    return NextResponse.json({
      apy: 10,
      timestamp: new Date().toISOString(),
      source: 'fallback',
      error: error
    }, {
      status: 300, // Still return 300 with fallback data
    });
  }
}