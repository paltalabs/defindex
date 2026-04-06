"use client"

import Footer from "@/components/globals/Footer"
import Navbar from "@/components/globals/navbar/Navbar"
import Image from "next/image"

export default function TermsOfService(){
  return (
    <div className="min-h-screen w-full bg-black relative z-0 overflow-x-hidden">
      <Image
        width={1440}
        height={6797}
        className="w-full h-full inset-0 absolute -z-10 object-cover"
        src="/images/web-background.png"
        alt=""
      />
      <Navbar/>
      <main className="container mx-auto max-w-4xl px-4 sm:px-6 py-12 sm:py-16 lg:py-20 mt-20">
        <div className="bg-cyan-950/30 backdrop-blur-sm border border-cyan-800/30 rounded-2xl p-4 sm:p-8 lg:p-12 overflow-hidden">
          <header className="mb-12">
            <h1 className="font-manrope font-bold text-3xl text-white mb-4 text-pretty break-words">
              DeFindex — Terms of Service
            </h1>
          </header>

          <div className="space-y-8 text-white">
            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                1. Introduction
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  Welcome to <strong>DeFindex</strong> (&quot;DeFindex&quot;, &quot;we&quot;, &quot;us&quot;, or &quot;our&quot;).
                  These Terms of Service (&quot;Terms&quot;) govern your access to and use of the DeFindex decentralized application (the &quot;DApp&quot;) and application programming interface (the &quot;API&quot;).
                </p>
                <p className="break-words text-pretty">
                  By accessing or using DeFindex, you agree to be bound by these Terms. If you do not agree, do not use the DApp or API.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                2. Nature of the Service
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  DeFindex provides <strong>non-custodial software tools</strong> that enable users to:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Create and configure <strong>on-chain vaults</strong></li>
                  <li>(Where enabled) interact with vaults to <strong>deposit assets</strong></li>
                  <li>Use the <strong>API</strong> to generate or construct blockchain transactions</li>
                </ul>
                <p className="break-words text-pretty">
                  DeFindex <strong>does not</strong>:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Custody user funds</li>
                  <li>Control or execute transactions on behalf of users</li>
                  <li>Provide brokerage, exchange, or financial intermediation services</li>
                </ul>
                <p className="break-words text-pretty">
                  All transactions are executed <strong>directly by users</strong> through their own wallets and the relevant blockchain networks.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                3. Eligibility
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  You must be at least 18 years old (or the legal age in your jurisdiction) to use DeFindex.
                </p>
                <p className="break-words text-pretty">
                  By using the service, you represent and warrant that:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>You are legally permitted to use decentralized finance tools in your jurisdiction</li>
                  <li>You are not located in, or a resident of, any jurisdiction subject to comprehensive sanctions or where use of the service is prohibited</li>
                </ul>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                4. Non-Custodial Nature
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  DeFindex is a <strong>non-custodial platform</strong>.
                  We never take possession of your digital assets, private keys, or credentials.
                </p>
                <p className="break-words text-pretty">
                  You are solely responsible for:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Securing your wallet</li>
                  <li>Safeguarding private keys and seed phrases</li>
                  <li>Verifying every transaction before signing</li>
                </ul>
                <p className="break-words text-pretty">
                  DeFindex has <strong>no ability to recover funds</strong> lost due to user error, compromised keys, or incorrect transactions.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                5. API Use
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  The DeFindex API:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Only assists in <strong>creating or structuring transaction data</strong></li>
                  <li>Does not submit, execute, or authorize transactions</li>
                  <li>Does not act as an agent or intermediary</li>
                </ul>
                <p className="break-words text-pretty">
                  You remain fully responsible for:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Reviewing all transaction parameters</li>
                  <li>Deciding whether to sign and broadcast any transaction</li>
                </ul>
                <p className="break-words text-pretty">
                  Use of the API is at your own risk.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                6. No Financial, Legal, or Tax Advice
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  Nothing on DeFindex constitutes:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Financial advice</li>
                  <li>Investment advice</li>
                  <li>Legal advice</li>
                  <li>Tax advice</li>
                </ul>
                <p className="break-words text-pretty">
                  DeFindex does not:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Recommend strategies</li>
                  <li>Endorse specific vaults, assets, or protocols</li>
                  <li>Assess suitability for any user</li>
                </ul>
                <p className="break-words text-pretty">
                  You are solely responsible for your own decisions and for seeking professional advice where appropriate.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                7. No Fiduciary Relationship
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  DeFindex does not act as:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>A fiduciary</li>
                  <li>A broker</li>
                  <li>An agent</li>
                  <li>An advisor</li>
                </ul>
                <p className="break-words text-pretty">
                  No fiduciary, partnership, or agency relationship is created by your use of the platform.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                8. Risks of Using DeFi
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  By using DeFindex, you acknowledge and accept the risks inherent in decentralized finance, including but not limited to:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Smart contract bugs or exploits</li>
                  <li>Oracle failures</li>
                  <li>Blockchain congestion or outages</li>
                  <li>Market volatility</li>
                  <li>Protocol insolvency or governance failures</li>
                </ul>
                <p className="break-words text-pretty">
                  DeFindex is <strong>not responsible</strong> for losses resulting from these risks.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                9. No Endorsement of Vaults or Assets
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  DeFindex does not:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Vet, audit, or guarantee any vault, asset, or protocol</li>
                  <li>Represent that any vault is safe, profitable, or compliant</li>
                </ul>
                <p className="break-words text-pretty">
                  Any interaction with vaults is at your own risk.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                10. Geographic Restrictions and Sanctions Compliance
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  DeFindex restricts access in certain jurisdictions to comply with applicable laws and international sanctions.
                </p>
                <p className="break-words text-pretty">
                  By using the platform, you represent and warrant that you are not:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Located in, or</li>
                  <li>A resident of</li>
                </ul>
                <p className="break-words text-pretty">
                  any jurisdiction subject to comprehensive sanctions or regulatory restrictions.
                </p>
                <p className="break-words text-pretty">
                  Access may be restricted based on geolocation or other compliance controls.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                11. User Responsibilities
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  You agree to:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Comply with all applicable laws and regulations</li>
                  <li>Not use DeFindex for illegal activities, including money laundering or sanctions evasion</li>
                  <li>Ensure that your use of the platform is lawful in your jurisdiction, including tax and reporting obligations</li>
                </ul>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                12. Third-Party Services
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  DeFindex relies on third-party infrastructure, including:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Blockchain networks</li>
                  <li>Wallet providers</li>
                  <li>RPC services</li>
                  <li>Oracles and indexing services</li>
                </ul>
                <p className="break-words text-pretty">
                  DeFindex is <strong>not responsible</strong> for:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Failures or outages of third-party services</li>
                  <li>Losses caused by third-party software or infrastructure</li>
                </ul>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                13. Intellectual Property
              </h2>
              <p className="text-gray-200 font-inter leading-relaxed text-pretty break-words">
                The DeFindex software may be open-source and licensed under applicable open-source licenses.
                Nothing in these Terms grants you any rights beyond those expressly provided in such licenses.
              </p>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                14. Disclaimer of Warranties
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  DeFindex is provided on an <strong>&quot;AS IS&quot; and &quot;AS AVAILABLE&quot;</strong> basis.
                </p>
                <p className="break-words text-pretty">
                  To the maximum extent permitted by law, we disclaim all warranties, including:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Merchantability</li>
                  <li>Fitness for a particular purpose</li>
                  <li>Non-infringement</li>
                  <li>Accuracy or reliability of the platform</li>
                </ul>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                15. Limitation of Liability
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  To the maximum extent permitted by law, DeFindex shall not be liable for any:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Direct, indirect, incidental, or consequential damages</li>
                  <li>Loss of funds, profits, or data</li>
                </ul>
                <p className="break-words text-pretty">
                  arising from your use of the DApp, API, or any interaction with vaults or smart contracts.
                </p>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                16. Indemnification
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  You agree to indemnify and hold harmless DeFindex, its contributors, and affiliates from any claims, damages, or liabilities arising from:
                </p>
                <ul className="list-disc list-inside space-y-1 ml-4 text-pretty">
                  <li>Your use of the platform</li>
                  <li>Your violation of these Terms</li>
                  <li>Your violation of any law or third-party rights</li>
                </ul>
              </div>
            </section>

            <section>
              <h2 className="font-manrope font-bold text-lg sm:text-xl lg:text-2xl text-lime-200 mb-4 text-pretty break-words">
                17. Modifications to the Service and Terms
              </h2>
              <div className="space-y-4 text-gray-200 font-inter leading-relaxed text-pretty">
                <p className="break-words text-pretty">
                  We may modify or discontinue any part of DeFindex at any time without notice.
                </p>
                <p className="break-words text-pretty">
                  We may update these Terms from time to time.
                  Continued use of the platform constitutes acceptance of the updated Terms.
                </p>
              </div>
            </section>

          </div>
        </div>
      </main>
      <Footer />
    </div>
  )
}
