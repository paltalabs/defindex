import { defindexClient } from '@/lib/server/defindexClient';
import { SupportedNetworks } from '@defindex/sdk';
import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  try {
    const vaultId = request.headers.get('vaultId');
    const network = request.headers.get('network') ?? 'mainnet';

    if (!vaultId) {
      return NextResponse.json(
        { error: 'vaultId header is required' },
        { status: 400 }
      );
    }

    const vaultInfo = await defindexClient.getVaultInfo(
      vaultId,
      network as SupportedNetworks
    );

    return NextResponse.json({ data: vaultInfo });
  } catch (error: unknown) {
    console.error('Error fetching vault info:', error);
    const statusCode = (error as { statusCode?: number }).statusCode ?? 500;
    const message = error instanceof Error ? error.message : 'Failed to fetch vault info';
    return NextResponse.json(
      { error: message },
      { status: statusCode }
    );
  }
}
