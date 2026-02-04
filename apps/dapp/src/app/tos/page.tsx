import type { Metadata } from "next";
import { Box, Heading, Text, VStack, List } from "@chakra-ui/react";

export const metadata: Metadata = {
  title: "Terms of Service | DeFindex",
  description: "DeFindex Terms of Service",
};

export default function TermsOfServicePage() {
  return (
    <Box
      maxW="4xl"
      mx="auto"
      px={{ base: 4, md: 8 }}
      py={{ base: 8, md: 12 }}
      color="white"
      mt="20"
    >
      <VStack gap={8} align="stretch">
        <Heading
          as="h1"
          size="2xl"
          fontFamily="var(--font-familjen-grotesk)"
          textAlign="center"
        >
          DeFindex — Terms of Service
        </Heading>

        <Section title="1. Introduction">
          <Text>
            Welcome to <strong>DeFindex</strong> (&quot;DeFindex&quot;,
            &quot;we&quot;, &quot;us&quot;, or &quot;our&quot;). These Terms of
            Service (&quot;Terms&quot;) govern your access to and use of the
            DeFindex decentralized application (the &quot;DApp&quot;) and
            application programming interface (the &quot;API&quot;).
          </Text>
          <Text>
            By accessing or using DeFindex, you agree to be bound by these
            Terms. If you do not agree, do not use the DApp or API.
          </Text>
        </Section>

        <Section title="2. Nature of the Service">
          <Text>
            DeFindex provides <strong>non-custodial software tools</strong> that
            enable users to:
          </Text>
          <List.Root ps={4}>
            <List.Item>
              Create and configure <strong>on-chain vaults</strong>
            </List.Item>
            <List.Item>
              (Where enabled) interact with vaults to{" "}
              <strong>deposit assets</strong>
            </List.Item>
            <List.Item>
              Use the <strong>API</strong> to generate or construct blockchain
              transactions
            </List.Item>
          </List.Root>
          <Text>
            DeFindex <strong>does not</strong>:
          </Text>
          <List.Root ps={4}>
            <List.Item>Custody user funds</List.Item>
            <List.Item>
              Control or execute transactions on behalf of users
            </List.Item>
            <List.Item>
              Provide brokerage, exchange, or financial intermediation services
            </List.Item>
          </List.Root>
          <Text>
            All transactions are executed <strong>directly by users</strong>{" "}
            through their own wallets and the relevant blockchain networks.
          </Text>
        </Section>

        <Section title="3. Eligibility">
          <Text>
            You must be at least 18 years old (or the legal age in your
            jurisdiction) to use DeFindex.
          </Text>
          <Text>By using the service, you represent and warrant that:</Text>
          <List.Root ps={4}>
            <List.Item>
              You are legally permitted to use decentralized finance tools in
              your jurisdiction
            </List.Item>
            <List.Item>
              You are not located in, or a resident of, any jurisdiction subject
              to comprehensive sanctions or where use of the service is
              prohibited
            </List.Item>
          </List.Root>
        </Section>

        <Section title="4. Non-Custodial Nature">
          <Text>
            DeFindex is a <strong>non-custodial platform</strong>. We never take
            possession of your digital assets, private keys, or credentials.
          </Text>
          <Text>You are solely responsible for:</Text>
          <List.Root ps={4}>
            <List.Item>Securing your wallet</List.Item>
            <List.Item>Safeguarding private keys and seed phrases</List.Item>
            <List.Item>Verifying every transaction before signing</List.Item>
          </List.Root>
          <Text>
            DeFindex has <strong>no ability to recover funds</strong> lost due
            to user error, compromised keys, or incorrect transactions.
          </Text>
        </Section>

        <Section title="5. API Use">
          <Text>The DeFindex API:</Text>
          <List.Root ps={4}>
            <List.Item>
              Only assists in{" "}
              <strong>creating or structuring transaction data</strong>
            </List.Item>
            <List.Item>
              Does not submit, execute, or authorize transactions
            </List.Item>
            <List.Item>Does not act as an agent or intermediary</List.Item>
          </List.Root>
          <Text>You remain fully responsible for:</Text>
          <List.Root ps={4}>
            <List.Item>Reviewing all transaction parameters</List.Item>
            <List.Item>
              Deciding whether to sign and broadcast any transaction
            </List.Item>
          </List.Root>
          <Text>Use of the API is at your own risk.</Text>
        </Section>

        <Section title="6. No Financial, Legal, or Tax Advice">
          <Text>Nothing on DeFindex constitutes:</Text>
          <List.Root ps={4}>
            <List.Item>Financial advice</List.Item>
            <List.Item>Investment advice</List.Item>
            <List.Item>Legal advice</List.Item>
            <List.Item>Tax advice</List.Item>
          </List.Root>
          <Text>DeFindex does not:</Text>
          <List.Root ps={4}>
            <List.Item>Recommend strategies</List.Item>
            <List.Item>
              Endorse specific vaults, assets, or protocols
            </List.Item>
            <List.Item>Assess suitability for any user</List.Item>
          </List.Root>
          <Text>
            You are solely responsible for your own decisions and for seeking
            professional advice where appropriate.
          </Text>
        </Section>

        <Section title="7. No Fiduciary Relationship">
          <Text>DeFindex does not act as:</Text>
          <List.Root ps={4}>
            <List.Item>A fiduciary</List.Item>
            <List.Item>A broker</List.Item>
            <List.Item>An agent</List.Item>
            <List.Item>An advisor</List.Item>
          </List.Root>
          <Text>
            No fiduciary, partnership, or agency relationship is created by your
            use of the platform.
          </Text>
        </Section>

        <Section title="8. Risks of Using DeFi">
          <Text>
            By using DeFindex, you acknowledge and accept the risks inherent in
            decentralized finance, including but not limited to:
          </Text>
          <List.Root ps={4}>
            <List.Item>Smart contract bugs or exploits</List.Item>
            <List.Item>Oracle failures</List.Item>
            <List.Item>Blockchain congestion or outages</List.Item>
            <List.Item>Market volatility</List.Item>
            <List.Item>Protocol insolvency or governance failures</List.Item>
          </List.Root>
          <Text>
            DeFindex is <strong>not responsible</strong> for losses resulting
            from these risks.
          </Text>
        </Section>

        <Section title="9. No Endorsement of Vaults or Assets">
          <Text>DeFindex does not:</Text>
          <List.Root ps={4}>
            <List.Item>
              Vet, audit, or guarantee any vault, asset, or protocol
            </List.Item>
            <List.Item>
              Represent that any vault is safe, profitable, or compliant
            </List.Item>
          </List.Root>
          <Text>Any interaction with vaults is at your own risk.</Text>
        </Section>

        <Section title="10. Geographic Restrictions and Sanctions Compliance">
          <Text>
            DeFindex restricts access in certain jurisdictions to comply with
            applicable laws and international sanctions.
          </Text>
          <Text>
            By using the platform, you represent and warrant that you are not:
          </Text>
          <List.Root ps={4}>
            <List.Item>Located in, or</List.Item>
            <List.Item>A resident of</List.Item>
          </List.Root>
          <Text>
            any jurisdiction subject to comprehensive sanctions or regulatory
            restrictions.
          </Text>
          <Text>
            Access may be restricted based on geolocation or other compliance
            controls.
          </Text>
        </Section>

        <Section title="11. User Responsibilities">
          <Text>You agree to:</Text>
          <List.Root ps={4}>
            <List.Item>
              Comply with all applicable laws and regulations
            </List.Item>
            <List.Item>
              Not use DeFindex for illegal activities, including money
              laundering or sanctions evasion
            </List.Item>
            <List.Item>
              Ensure that your use of the platform is lawful in your
              jurisdiction, including tax and reporting obligations
            </List.Item>
          </List.Root>
        </Section>

        <Section title="12. Third-Party Services">
          <Text>
            DeFindex relies on third-party infrastructure, including:
          </Text>
          <List.Root ps={4}>
            <List.Item>Blockchain networks</List.Item>
            <List.Item>Wallet providers</List.Item>
            <List.Item>RPC services</List.Item>
            <List.Item>Oracles and indexing services</List.Item>
          </List.Root>
          <Text>
            DeFindex is <strong>not responsible</strong> for:
          </Text>
          <List.Root ps={4}>
            <List.Item>Failures or outages of third-party services</List.Item>
            <List.Item>
              Losses caused by third-party software or infrastructure
            </List.Item>
          </List.Root>
        </Section>

        <Section title="13. Intellectual Property">
          <Text>
            The DeFindex software may be open-source and licensed under
            applicable open-source licenses.
          </Text>
          <Text>
            Nothing in these Terms grants you any rights beyond those expressly
            provided in such licenses.
          </Text>
        </Section>

        <Section title="14. Disclaimer of Warranties">
          <Text>
            DeFindex is provided on an{" "}
            <strong>&quot;AS IS&quot; and &quot;AS AVAILABLE&quot;</strong>{" "}
            basis.
          </Text>
          <Text>
            To the maximum extent permitted by law, we disclaim all warranties,
            including:
          </Text>
          <List.Root ps={4}>
            <List.Item>Merchantability</List.Item>
            <List.Item>Fitness for a particular purpose</List.Item>
            <List.Item>Non-infringement</List.Item>
            <List.Item>Accuracy or reliability of the platform</List.Item>
          </List.Root>
        </Section>

        <Section title="15. Limitation of Liability">
          <Text>
            To the maximum extent permitted by law, DeFindex shall not be liable
            for any:
          </Text>
          <List.Root ps={4}>
            <List.Item>
              Direct, indirect, incidental, or consequential damages
            </List.Item>
            <List.Item>Loss of funds, profits, or data</List.Item>
          </List.Root>
          <Text>
            arising from your use of the DApp, API, or any interaction with
            vaults or smart contracts.
          </Text>
        </Section>

        <Section title="16. Indemnification">
          <Text>
            You agree to indemnify and hold harmless DeFindex, its contributors,
            and affiliates from any claims, damages, or liabilities arising
            from:
          </Text>
          <List.Root ps={4}>
            <List.Item>Your use of the platform</List.Item>
            <List.Item>Your violation of these Terms</List.Item>
            <List.Item>Your violation of any law or third-party rights</List.Item>
          </List.Root>
        </Section>

        <Section title="17. Modifications to the Service and Terms">
          <Text>
            We may modify or discontinue any part of DeFindex at any time
            without notice.
          </Text>
          <Text>We may update these Terms from time to time.</Text>
          <Text>
            Continued use of the platform constitutes acceptance of the updated
            Terms.
          </Text>
        </Section>
      </VStack>
    </Box>
  );
}

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <VStack gap={3} align="stretch">
      <Heading
        as="h2"
        size="lg"
        fontFamily="var(--font-familjen-grotesk)"
        color="#d3ffb4"
      >
        {title}
      </Heading>
      <VStack gap={2} align="stretch" color="rgba(255, 255, 255, 0.85)">
        {children}
      </VStack>
    </VStack>
  );
}
