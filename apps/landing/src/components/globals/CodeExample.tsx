"use client";

import ScheduleDemoButton from "@/components/common/ScheduleDemoButton";
import Link from "next/link";
import { FiExternalLink } from "react-icons/fi";
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';
import GradientText from "../common/GradientText";

export default function CodeExample() {
    const codeSnippet = `import {
  DefindexSDK,
  SupportedNetworks,
  DepositToVaultParams
} from '@defindex/sdk';

// Initialize the DefindexSDK
const sdk = new DefindexSDK({
  apiKey: 'sk_your_api_key_here',
  baseUrl: 'https://api.defindex.io'
});

const vaultAddress = 'CAEJL2XKGLSWCPKSVVRYAWLQKE4DS24YCZX53CLUMWGOVEOERSAZH5UM';
const userAddress = 'GUSER_ADDRESS...';

async function performDepositAndSendTransaction() {
  try {
    // 1. Generate a Deposit Transaction
    const depositData: DepositToVaultParams = {
      amounts: [1000000], // Amount in smallest unit
      caller: userAddress,
      invest: true,       // Auto-invest after deposit
      slippageBps: 100    // 1% slippage tolerance
    };

    const depositResponse = await sdk.depositToVault(
      vaultAddress,
      depositData,
      SupportedNetworks.TESTNET
    );

    // 2. Sign the transaction (with user's wallet)
    const signedXDR = await yourWallet.sign(depositResponse.xdr);

    // 3. Send the transaction
    const result = await sdk.sendTransaction(
      signedXDR,
      SupportedNetworks.TESTNET,
      false
    );

    console.log('Transaction hash:', result.hash);
    console.log('Status:', result.status);

  } catch (error) {
    console.error('Deposit failed:', error);
  }
}`;

    return (
        <section id="quick-deposit-example" className="py-10 px-4 md:px-8 lg:px-16 overflow-hidden w-full">
            <div className="max-w-full mx-auto bg-white rounded-3xl p-4 sm:p-8 md:p-16 w-full" style={{maxWidth: 'calc(100vw - 2rem)'}}>
                <div className="flex flex-col items-center text-center gap-8 sm:gap-12">
                    {/* Left side - Text and buttons */}
                    <div className="w-full px-2 sm:px-4 grid grid-cols-2">
                        <div className="col-span-2 xl:col-span-1">
                            <GradientText
                                as="h3"
                                variant="green"
                                className="font-familjen-grotesk text-left text-[38px] md:text-[48px] lg:text-[72px] font-normal mb-4 sm:mb-6 col-span-1"
                                style={{
                                    fontWeight: 700,
                                    lineHeight: '1em',
                                }}
                            >
                                Quick Deposit Example
                            </GradientText>
                            <p className="font-inter text-left text-sm sm:text-base md:text-[24px] text-cyan-950/80 mr-16">
                                Initialize SDK, generate deposit transaction, and send it to the network.
                            </p>
                        </div>
                        <div className="flex mt-6 col-span-2 xl:col-span-1 flex-col sm:flex-row gap-4 items-end justify-center ">
                            <ScheduleDemoButton className="w-full sm:w-auto max-h-16" />
                            <Link
                                href="https://github.com/paltalabs/defindex-sdk"
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label="View DeFindex SDK source code on GitHub"
                                className="flex items-center justify-center bg-cyan-950 text-lime-200 font-manrope font-extrabold text-sm rounded-3xl px-6 py-4 transition-all duration-normal hover:scale-105 hover:bg-cyan-950/70 hover:shadow-lg active:scale-95"
                            >
                                Explore SDK on GitHub
                                <FiExternalLink className="text-lime-200 text-xs lg:text-xs transition-colors duration-normal group-hover:text-lime-200 ml-2" style={{filter: 'drop-shadow(0 2px 4px rgba(0,0,0,0.6))'}} />
                            </Link>
                        </div>
                    </div>

                    {/* Right side - Code block */}
                    <div className="relative w-full overflow-x-auto">
                        <div className="rounded-2xl border-2 overflow-hidden min-w-0">
                            <SyntaxHighlighter
                                language="typescript"
                                style={vscDarkPlus}
                                customStyle={{
                                    margin: 0,
                                    padding: '1rem',
                                    background: 'linear-gradient(136deg, #033036 0%, rgba(32, 33, 36, 1) 70%)',
                                    fontSize: 'clamp(0.65rem, 2vw, 0.875rem)',
                                    lineHeight: '1.5',
                                    borderRadius: '1rem',
                                    maxWidth: '100%',
                                    overflowX: 'auto'
                                }}
                                wrapLongLines={false}
                                showLineNumbers={false}
                            >
                                {codeSnippet}
                            </SyntaxHighlighter>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}