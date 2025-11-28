import { defindexClient } from '@/lib/defindexClient';
import { CreateDefindexVault, CreateVaultAutoInvestParams, SupportedNetworks } from '@defindex/sdk';
import { NextRequest, NextResponse } from 'next/server';

export async function POST(request: NextRequest) {
  try {
    const body = await request.json();
    const { vaultConfig, network, withDeposit } = body;

    if (!vaultConfig) {
      return NextResponse.json(
        { error: 'vaultConfig is required' },
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

    let result;
    if (withDeposit) {
      result = await defindexClient.createVaultAutoInvest(
        vaultConfig as CreateVaultAutoInvestParams,
        sdkNetwork
      );
    } else {
      result = await defindexClient.createVault(
        vaultConfig as CreateDefindexVault,
        sdkNetwork
      );
    }

    return NextResponse.json({ data: result });
  } catch (error) {
    console.error('Error creating vault:', error);
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Failed to create vault' },
      { status: 500 }
    );
  }
}
