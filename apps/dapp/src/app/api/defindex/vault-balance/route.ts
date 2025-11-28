import { NextRequest, NextResponse } from 'next/server';
import { defindexClient } from '@/lib/defindexClient';
import { SupportedNetworks } from '@defindex/sdk';

export async function GET(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const vaultAddress = searchParams.get('vaultAddress');
    const userAddress = searchParams.get('userAddress');
    const network = searchParams.get('network') as 'mainnet' | 'testnet';

    if (!vaultAddress) {
      return NextResponse.json(
        { error: 'vaultAddress is required' },
        { status: 400 }
      );
    }

    if (!userAddress) {
      return NextResponse.json(
        { error: 'userAddress is required' },
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

    const balance = await defindexClient.getVaultBalance(
      vaultAddress,
      userAddress,
      sdkNetwork
    );

    return NextResponse.json({ data: balance });
  } catch (error) {
    console.error('Error fetching vault balance:', error);
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Failed to fetch vault balance' },
      { status: 500 }
    );
  }
}
