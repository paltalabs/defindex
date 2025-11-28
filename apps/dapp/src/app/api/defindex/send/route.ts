import { NextRequest, NextResponse } from 'next/server';
import { defindexClient } from '@/lib/defindexClient';
import { SupportedNetworks } from '@defindex/sdk';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { signedXdr, network } = body;

    if (!signedXdr) {
      return NextResponse.json(
        { error: 'signedXdr is required' },
        { status: 400 }
      );
    }

    if (!network || !['mainnet', 'testnet'].includes(network)) {
      return NextResponse.json(
        { error: 'network must be mainnet or testnet' },
        { status: 400 }
      );
    }

    const sdkNetwork = network === 'mainnet'
      ? SupportedNetworks.MAINNET
      : SupportedNetworks.TESTNET;

    const result = await defindexClient.sendTransaction(signedXdr, sdkNetwork);

    return NextResponse.json({ data: result });
  } catch (error) {
    console.error('Error sending transaction:', error);
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Failed to send transaction' },
      { status: 500 }
    );
  }
}
