import { useSorobanReact, contractInvoke } from "stellar-react";
import { HStack, Text } from "@chakra-ui/react";
import { Address, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { useEffect, useState } from "react";
import { Button } from "./ui/button";

export const TestTokens = () => {
  const sorobanContext = useSorobanReact();
  const { address, activeNetwork: networkPassphrase, sorobanServer: server } = sorobanContext;
  const [isSubmitting, setSubmitting] = useState(false);
  const [balance, setBalance] = useState<string>("0");

  const testnetUSDC = "CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI";
  const admin_account = Keypair.fromSecret(
    process.env.NEXT_PUBLIC_TEST_TOKENS_ADMIN as string,
  );

  const fetchBalance = async () => { 
    if (!address) return;
    contractInvoke({
      contractAddress: testnetUSDC,
      method: 'balance',
      args: [new Address(address).toScVal()],
      sorobanContext,
      signAndSend: false,
    }).then((result) => { 
      let balance = scValToNative(result as xdr.ScVal);
      balance = BigInt(BigInt(balance) / BigInt(1e7)).toString();
      setBalance(balance);

    }).catch ((error) => { 
      console.log('🚀 « error:', error);
    })
  }

  useEffect(() => { 
    fetchBalance();
  }, [address])

  const handleMint = async () => {
    console.log("Minting");
    setSubmitting(true);

    const amount = 1000000000000

    let adminSource;

    try {
      adminSource = await server?.getAccount(admin_account.publicKey());
    } catch (error) {
      alert('Your wallet or the token admin wallet might not be funded');
      setSubmitting(false);
      return;
    }

    if (!address) {
      return;
    }
    if (!adminSource) {
      return;
    }

    try {
      let result = await contractInvoke({
        contractAddress: testnetUSDC,
        method: 'mint',
        args: [new Address(address).toScVal(), nativeToScVal(amount, {type: 'i128'})],
        sorobanContext,
        signAndSend: true,
        secretKey: admin_account.secret(),
      });
      if (result) {
        fetchBalance();
      }
    } catch (error) {
      console.log('🚀 « error:', error);
    }

    setSubmitting(false);
  }

  return (
    <HStack>
      <Text>Current Balance: {balance} USDC</Text>
      <Button
        rounded={18}
        onClick={handleMint}
        loadingText="Minting..."
        loading={isSubmitting}
      >
        Mint test USDC
      </Button>
    </HStack>
  );
};
